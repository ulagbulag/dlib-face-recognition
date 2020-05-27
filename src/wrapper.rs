cpp! {{
    #include <dlib/image_processing/frontal_face_detector.h>
    #include <dlib/image_processing/full_object_detection.h>
    #include <dlib/dnn.h>

    using namespace dlib;

    // face encoding network definition from
    // https://github.com/davisking/dlib/blob/master/tools/python/src/face_recognition.cpp

    template <template <int,template<typename>class,int,typename> class block, int N, template<typename>class BN, typename SUBNET>
    using residual = add_prev1<block<N,BN,1,tag1<SUBNET>>>;

    template <template <int,template<typename>class,int,typename> class block, int N, template<typename>class BN, typename SUBNET>
    using residual_down = add_prev2<avg_pool<2,2,2,2,skip1<tag2<block<N,BN,2,tag1<SUBNET>>>>>>;

    template <int N, template <typename> class BN, int stride, typename SUBNET>
    using block  = BN<con<N,3,3,1,1,relu<BN<con<N,3,3,stride,stride,SUBNET>>>>>;

    template <int N, typename SUBNET> using ares      = relu<residual<block,N,affine,SUBNET>>;
    template <int N, typename SUBNET> using ares_down = relu<residual_down<block,N,affine,SUBNET>>;

    template <typename SUBNET> using alevel0 = ares_down<256,SUBNET>;
    template <typename SUBNET> using alevel1 = ares<256,ares<256,ares_down<256,SUBNET>>>;
    template <typename SUBNET> using alevel2 = ares<128,ares<128,ares_down<128,SUBNET>>>;
    template <typename SUBNET> using alevel3 = ares<64,ares<64,ares<64,ares_down<64,SUBNET>>>>;
    template <typename SUBNET> using alevel4 = ares<32,ares<32,ares<32,SUBNET>>>;

    using face_encoding_nn = loss_metric<fc_no_bias<128,avg_pool_everything<
                                alevel0<
                                alevel1<
                                alevel2<
                                alevel3<
                                alevel4<
                                max_pool<3,3,2,2,relu<affine<con<32,7,7,2,2,
                                input_rgb_image_sized<150>
                                >>>>>>>>>>>>;

    // cnn face detector definition from
    // https://github.com/davisking/dlib/blob/master/tools/python/src/cnn_face_detector.cpp#L121

    template <long num_filters, typename SUBNET> using con5d = con<num_filters,5,5,2,2,SUBNET>;
    template <long num_filters, typename SUBNET> using con5  = con<num_filters,5,5,1,1,SUBNET>;

    template <typename SUBNET> using downsampler  = relu<affine<con5d<32, relu<affine<con5d<32, relu<affine<con5d<16,SUBNET>>>>>>>>>;
    template <typename SUBNET> using rcon5  = relu<affine<con5<45,SUBNET>>>;

    using face_detection_cnn = loss_mmod<con<1,9,9,1,1,rcon5<rcon5<rcon5<downsampler<input_rgb_image_pyramid<pyramid_down<6>>>>>>>>;

    // misc

    // TODO: I am unsure if having rnd as a global here is thread safe.

    dlib::rand rnd;

    // https://github.com/davisking/dlib/blob/master/tools/python/src/face_recognition.cpp#L131
    std::vector<matrix<rgb_pixel>> jitter_image(const matrix<rgb_pixel>& img, const int num_jitters) {
        std::vector<matrix<rgb_pixel>> crops;
        for (int i = 0; i < num_jitters; ++i) {
            crops.push_back(dlib::jitter_image(img, rnd));
        }
        return crops;
    }
}}
