cpp! {{
    #include <dlib/dnn.h>
    #include <dlib/image_processing/frontal_face_detector.h>
    #include <dlib/image_processing/full_object_detection.h>
    #include <dlib/image_transforms.h>
    #include <dlib/matrix/matrix_math_functions_abstract.h>

    // face encoding network definition from
    // https://github.com/davisking/dlib/blob/master/tools/python/src/face_recognition.cpp

    template <template <int,template<typename>class,int,typename> class block, int N, template<typename>class BN, typename SUBNET>
    using residual = dlib::add_prev1<block<N,BN,1,dlib::tag1<SUBNET>>>;

    template <template <int,template<typename>class,int,typename> class block, int N, template<typename>class BN, typename SUBNET>
    using residual_down = dlib::add_prev2<dlib::avg_pool<2,2,2,2,dlib::skip1<dlib::tag2<block<N,BN,2,dlib::tag1<SUBNET>>>>>>;

    template <int N, template <typename> class BN, int stride, typename SUBNET>
    using block  = BN<dlib::con<N,3,3,1,1,dlib::relu<BN<dlib::con<N,3,3,stride,stride,SUBNET>>>>>;

    template <int N, typename SUBNET> using ares      = dlib::relu<residual<block,N,dlib::affine,SUBNET>>;
    template <int N, typename SUBNET> using ares_down = dlib::relu<residual_down<block,N,dlib::affine,SUBNET>>;

    template <typename SUBNET> using alevel0 = ares_down<256,SUBNET>;
    template <typename SUBNET> using alevel1 = ares<256,ares<256,ares_down<256,SUBNET>>>;
    template <typename SUBNET> using alevel2 = ares<128,ares<128,ares_down<128,SUBNET>>>;
    template <typename SUBNET> using alevel3 = ares<64,ares<64,ares<64,ares_down<64,SUBNET>>>>;
    template <typename SUBNET> using alevel4 = ares<32,ares<32,ares<32,SUBNET>>>;

    using face_encoding_nn = dlib::loss_metric<dlib::fc_no_bias<128,dlib::avg_pool_everything<
                                alevel0<
                                alevel1<
                                alevel2<
                                alevel3<
                                alevel4<
                                dlib::max_pool<3,3,2,2,dlib::relu<dlib::affine<dlib::con<32,7,7,2,2,
                                dlib::input_rgb_image_sized<150>
                                >>>>>>>>>>>>;

    // cnn face detector definition from
    // https://github.com/davisking/dlib/blob/master/tools/python/src/cnn_face_detector.cpp#L121

    template <long num_filters, typename SUBNET> using con5d = dlib::con<num_filters,5,5,2,2,SUBNET>;
    template <long num_filters, typename SUBNET> using con5  = dlib::con<num_filters,5,5,1,1,SUBNET>;

    template <typename SUBNET> using downsampler  = dlib::relu<dlib::affine<con5d<32, dlib::relu<dlib::affine<con5d<32, dlib::relu<dlib::affine<con5d<16,SUBNET>>>>>>>>>;
    template <typename SUBNET> using rcon5  = dlib::relu<dlib::affine<con5<45,SUBNET>>>;

    using face_detection_cnn = dlib::loss_mmod<dlib::con<1,9,9,1,1,rcon5<rcon5<rcon5<downsampler<dlib::input_rgb_image_pyramid<dlib::pyramid_down<6>>>>>>>>;

    // misc

    // TODO: I am unsure if having rnd as a global here is thread safe.

    dlib::rand rnd;

    // https://github.com/davisking/dlib/blob/master/tools/python/src/face_recognition.cpp#L131
    std::vector<dlib::matrix<dlib::rgb_pixel>> jitter_image(const dlib::matrix<dlib::rgb_pixel>& img, const int num_jitters) {
        std::vector<dlib::matrix<dlib::rgb_pixel>> crops;
        for (int i = 0; i < num_jitters; ++i) {
            crops.push_back(dlib::jitter_image(img, rnd));
        }
        return crops;
    }
}}
