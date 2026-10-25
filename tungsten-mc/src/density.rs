pub mod density_function {
    // Auto-generated inline Rust module from RCL
    #[allow(dead_code)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(warnings)]
    #[allow(clippy::all)]
    
    use super::*;
    use super::orchestration::*;
    
    const spline_coordinates_85: [f32; 2] = [-0.2_f32, 0.2_f32];
    const spline_coordinates_86: [f32; 2] = [-0.05_f32, 0.05_f32];
    const spline_coordinates_87: [f32; 2] = [-0.05_f32, 0.05_f32];
    const spline_coordinates_88: [f32; 2] = [0_f32, 0.1_f32];
    const spline_derivatives_88: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_89: [f32; 2] = [-0.9_f32, -0.69_f32];
    const spline_derivatives_85: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_86: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_87: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_89: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_90: [f32; 10] = [-0.6_f32, -0.5_f32, -0.35_f32, -0.25_f32, -0.1_f32, 0.03_f32, 0.35_f32, 0.45_f32, 0.55_f32, 0.62_f32];
    const spline_coordinates_91: [f32; 2] = [-0.2_f32, 0.2_f32];
    const spline_coordinates_92: [f32; 2] = [0_f32, 0.1_f32];
    const spline_derivatives_92: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_93: [f32; 2] = [-0.9_f32, -0.69_f32];
    const spline_derivatives_91: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_93: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_94: [f32; 10] = [-0.6_f32, -0.5_f32, -0.35_f32, -0.25_f32, -0.1_f32, 0.03_f32, 0.35_f32, 0.45_f32, 0.55_f32, 0.62_f32];
    const spline_coordinates_95: [f32; 2] = [-0.2_f32, 0.2_f32];
    const spline_coordinates_96: [f32; 2] = [0_f32, 0.1_f32];
    const spline_derivatives_96: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_97: [f32; 2] = [-0.9_f32, -0.69_f32];
    const spline_derivatives_95: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_97: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_98: [f32; 10] = [-0.6_f32, -0.5_f32, -0.35_f32, -0.25_f32, -0.1_f32, 0.03_f32, 0.35_f32, 0.45_f32, 0.55_f32, 0.62_f32];
    const spline_coordinates_99: [f32; 2] = [-0.2_f32, 0.2_f32];
    const spline_derivatives_99: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_100: [f32; 2] = [0.45_f32, 0.7_f32];
    const spline_coordinates_101: [f32; 2] = [-0.7_f32, -0.15_f32];
    const spline_derivatives_100: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_101: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_102: [f32; 11] = [-0.6_f32, -0.5_f32, -0.35_f32, -0.25_f32, -0.1_f32, 0.03_f32, 0.05_f32, 0.4_f32, 0.45_f32, 0.55_f32, 0.58_f32];
    const spline_derivatives_90: [f32; 10] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_94: [f32; 10] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_98: [f32; 10] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_102: [f32; 11] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_103: [f32; 5] = [-0.19_f32, -0.15_f32, -0.1_f32, 0.03_f32, 0.06_f32];
    const spline_derivatives_103: [f32; 5] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_56: [f32; 2] = [-1_f32, 1_f32];
    const spline_coordinates_57: [f32; 2] = [-1_f32, 1_f32];
    const spline_coordinates_58: [f32; 6] = [-1_f32, -0.75_f32, -0.65_f32, 0.5954547_f32, 0.6054547_f32, 1_f32];
    const spline_coordinates_59: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_coordinates_60: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_coordinates_61: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_coordinates_62: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_derivatives_56: [f32; 2] = [0.38940096_f32, 0.38940096_f32];
    const spline_derivatives_57: [f32; 2] = [0.37788022_f32, 0.37788022_f32];
    const spline_derivatives_58: [f32; 6] = [0_f32, 0_f32, 0_f32, 0_f32, 0.2534563_f32, 0.2534563_f32];
    const spline_derivatives_59: [f32; 5] = [0.5_f32, 0_f32, 0_f32, 0_f32, 0.007000001_f32];
    const spline_derivatives_60: [f32; 5] = [0.5_f32, 0_f32, 0_f32, 0.1_f32, 0.007000001_f32];
    const spline_derivatives_61: [f32; 5] = [0.5_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_62: [f32; 5] = [0_f32, 0_f32, 0_f32, 0.06_f32, 0_f32];
    const spline_coordinates_63: [f32; 7] = [-0.85_f32, -0.7_f32, -0.4_f32, -0.35_f32, -0.1_f32, 0.2_f32, 0.7_f32];
    const spline_coordinates_64: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_coordinates_65: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_coordinates_66: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_coordinates_67: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_derivatives_64: [f32; 5] = [0.5_f32, 0_f32, 0_f32, 0_f32, 0.007000001_f32];
    const spline_derivatives_65: [f32; 5] = [0.5_f32, 0.01_f32, 0.01_f32, 0.094000004_f32, 0.007000001_f32];
    const spline_derivatives_66: [f32; 5] = [0.5_f32, 0_f32, 0_f32, 0.04_f32, 0.049_f32];
    const spline_derivatives_67: [f32; 5] = [0_f32, 0_f32, 0_f32, 0.12_f32, 0.049_f32];
    const spline_coordinates_68: [f32; 7] = [-0.85_f32, -0.7_f32, -0.4_f32, -0.35_f32, -0.1_f32, 0.2_f32, 0.7_f32];
    const spline_coordinates_69: [f32; 3] = [-1_f32, 0_f32, 1_f32];
    const spline_coordinates_70: [f32; 3] = [-1_f32, 0_f32, 1_f32];
    const spline_coordinates_71: [f32; 3] = [-1_f32, 0_f32, 1_f32];
    const spline_coordinates_72: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_coordinates_73: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_coordinates_74: [f32; 3] = [-1_f32, -0.4_f32, 0_f32];
    const spline_derivatives_69: [f32; 3] = [0_f32, 0.5138249_f32, 0.5138249_f32];
    const spline_derivatives_70: [f32; 3] = [0_f32, 0.43317974_f32, 0.43317974_f32];
    const spline_derivatives_71: [f32; 3] = [0_f32, 0.3917051_f32, 0.3917051_f32];
    const spline_derivatives_72: [f32; 5] = [0.5_f32, 0_f32, 0_f32, 0_f32, 0.049000014_f32];
    const spline_derivatives_73: [f32; 5] = [0.5_f32, 0.07_f32, 0.07_f32, 0.658_f32, 0.049000014_f32];
    const spline_derivatives_74: [f32; 3] = [0_f32, 0_f32, 0_f32];
    const spline_coordinates_75: [f32; 11] = [-0.85_f32, -0.7_f32, -0.4_f32, -0.35_f32, -0.1_f32, 0.2_f32, 0.4_f32, 0.45_f32, 0.55_f32, 0.58_f32, 0.7_f32];
    const spline_coordinates_76: [f32; 3] = [-1_f32, 0_f32, 1_f32];
    const spline_coordinates_77: [f32; 3] = [-1_f32, 0_f32, 1_f32];
    const spline_coordinates_78: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_coordinates_79: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_coordinates_80: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_derivatives_80: [f32; 5] = [0.5_f32, 0_f32, 0_f32, 0.04_f32, 0.049_f32];
    const spline_coordinates_81: [f32; 3] = [-1_f32, -0.4_f32, 0_f32];
    const spline_coordinates_82: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_derivatives_76: [f32; 3] = [0_f32, 0.5760369_f32, 0.5760369_f32];
    const spline_derivatives_77: [f32; 3] = [0_f32, 0.4608295_f32, 0.4608295_f32];
    const spline_derivatives_78: [f32; 5] = [0.5_f32, 0_f32, 0_f32, 0_f32, 0.070000015_f32];
    const spline_derivatives_79: [f32; 5] = [0.5_f32, 0.099999994_f32, 0.099999994_f32, 0.94_f32, 0.070000015_f32];
    const spline_derivatives_81: [f32; 3] = [0_f32, 0_f32, 0_f32];
    const spline_derivatives_82: [f32; 5] = [0.015_f32, 0_f32, 0_f32, 0.04_f32, 0.049_f32];
    const spline_coordinates_83: [f32; 11] = [-0.85_f32, -0.7_f32, -0.4_f32, -0.35_f32, -0.1_f32, 0.2_f32, 0.4_f32, 0.45_f32, 0.55_f32, 0.58_f32, 0.7_f32];
    const spline_derivatives_63: [f32; 7] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_68: [f32; 7] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_75: [f32; 11] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_83: [f32; 11] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_84: [f32; 10] = [-1.1_f32, -1.02_f32, -0.51_f32, -0.44_f32, -0.18_f32, -0.16_f32, -0.15_f32, -0.1_f32, 0.25_f32, 1_f32];
    const spline_derivatives_84: [f32; 10] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates: [f32; 2] = [-1_f32, 1_f32];
    const spline_coordinates_1: [f32; 2] = [-1_f32, 1_f32];
    const spline_coordinates_2: [f32; 6] = [-1_f32, -0.75_f32, -0.65_f32, 0.5954547_f32, 0.6054547_f32, 1_f32];
    const spline_coordinates_3: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_coordinates_4: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_coordinates_5: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_coordinates_6: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_derivatives: [f32; 2] = [0.38940096_f32, 0.38940096_f32];
    const spline_derivatives_1: [f32; 2] = [0.37788022_f32, 0.37788022_f32];
    const spline_derivatives_2: [f32; 6] = [0_f32, 0_f32, 0_f32, 0_f32, 0.2534563_f32, 0.2534563_f32];
    const spline_derivatives_3: [f32; 5] = [0.5_f32, 0_f32, 0_f32, 0_f32, 0.007000001_f32];
    const spline_derivatives_4: [f32; 5] = [0.5_f32, 0_f32, 0_f32, 0.1_f32, 0.007000001_f32];
    const spline_derivatives_5: [f32; 5] = [0.5_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_6: [f32; 5] = [0_f32, 0_f32, 0_f32, 0.06_f32, 0_f32];
    const spline_coordinates_7: [f32; 7] = [-0.85_f32, -0.7_f32, -0.4_f32, -0.35_f32, -0.1_f32, 0.2_f32, 0.7_f32];
    const spline_coordinates_8: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_coordinates_9: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_coordinates_10: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_coordinates_11: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_derivatives_8: [f32; 5] = [0.5_f32, 0_f32, 0_f32, 0_f32, 0.007000001_f32];
    const spline_derivatives_9: [f32; 5] = [0.5_f32, 0.01_f32, 0.01_f32, 0.094000004_f32, 0.007000001_f32];
    const spline_derivatives_10: [f32; 5] = [0.5_f32, 0_f32, 0_f32, 0.04_f32, 0.049_f32];
    const spline_derivatives_11: [f32; 5] = [0_f32, 0_f32, 0_f32, 0.12_f32, 0.049_f32];
    const spline_coordinates_12: [f32; 7] = [-0.85_f32, -0.7_f32, -0.4_f32, -0.35_f32, -0.1_f32, 0.2_f32, 0.7_f32];
    const spline_coordinates_13: [f32; 3] = [-1_f32, 0_f32, 1_f32];
    const spline_coordinates_14: [f32; 3] = [-1_f32, 0_f32, 1_f32];
    const spline_coordinates_15: [f32; 3] = [-1_f32, 0_f32, 1_f32];
    const spline_coordinates_16: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_coordinates_17: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_coordinates_18: [f32; 3] = [-1_f32, -0.4_f32, 0_f32];
    const spline_derivatives_13: [f32; 3] = [0_f32, 0.5138249_f32, 0.5138249_f32];
    const spline_derivatives_14: [f32; 3] = [0_f32, 0.43317974_f32, 0.43317974_f32];
    const spline_derivatives_15: [f32; 3] = [0_f32, 0.3917051_f32, 0.3917051_f32];
    const spline_derivatives_16: [f32; 5] = [0.5_f32, 0_f32, 0_f32, 0_f32, 0.049000014_f32];
    const spline_derivatives_17: [f32; 5] = [0.5_f32, 0.07_f32, 0.07_f32, 0.658_f32, 0.049000014_f32];
    const spline_derivatives_18: [f32; 3] = [0_f32, 0_f32, 0_f32];
    const spline_coordinates_19: [f32; 11] = [-0.85_f32, -0.7_f32, -0.4_f32, -0.35_f32, -0.1_f32, 0.2_f32, 0.4_f32, 0.45_f32, 0.55_f32, 0.58_f32, 0.7_f32];
    const spline_coordinates_20: [f32; 3] = [-1_f32, 0_f32, 1_f32];
    const spline_coordinates_21: [f32; 3] = [-1_f32, 0_f32, 1_f32];
    const spline_coordinates_22: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_coordinates_23: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_coordinates_24: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_derivatives_24: [f32; 5] = [0.5_f32, 0_f32, 0_f32, 0.04_f32, 0.049_f32];
    const spline_coordinates_25: [f32; 3] = [-1_f32, -0.4_f32, 0_f32];
    const spline_coordinates_26: [f32; 5] = [-1_f32, -0.4_f32, 0_f32, 0.4_f32, 1_f32];
    const spline_derivatives_20: [f32; 3] = [0_f32, 0.5760369_f32, 0.5760369_f32];
    const spline_derivatives_21: [f32; 3] = [0_f32, 0.4608295_f32, 0.4608295_f32];
    const spline_derivatives_22: [f32; 5] = [0.5_f32, 0_f32, 0_f32, 0_f32, 0.070000015_f32];
    const spline_derivatives_23: [f32; 5] = [0.5_f32, 0.099999994_f32, 0.099999994_f32, 0.94_f32, 0.070000015_f32];
    const spline_derivatives_25: [f32; 3] = [0_f32, 0_f32, 0_f32];
    const spline_derivatives_26: [f32; 5] = [0.015_f32, 0_f32, 0_f32, 0.04_f32, 0.049_f32];
    const spline_coordinates_27: [f32; 11] = [-0.85_f32, -0.7_f32, -0.4_f32, -0.35_f32, -0.1_f32, 0.2_f32, 0.4_f32, 0.45_f32, 0.55_f32, 0.58_f32, 0.7_f32];
    const spline_derivatives_7: [f32; 7] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_12: [f32; 7] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_19: [f32; 11] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_27: [f32; 11] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_28: [f32; 10] = [-1.1_f32, -1.02_f32, -0.51_f32, -0.44_f32, -0.18_f32, -0.16_f32, -0.15_f32, -0.1_f32, 0.25_f32, 1_f32];
    const spline_derivatives_28: [f32; 10] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_29: [f32; 2] = [-0.01_f32, 0.01_f32];
    const spline_derivatives_29: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_30: [f32; 3] = [0.19999999_f32, 0.44999996_f32, 1_f32];
    const spline_coordinates_31: [f32; 2] = [-0.01_f32, 0.01_f32];
    const spline_derivatives_31: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_32: [f32; 3] = [0.19999999_f32, 0.44999996_f32, 1_f32];
    const spline_derivatives_30: [f32; 3] = [0_f32, 0_f32, 0_f32];
    const spline_derivatives_32: [f32; 3] = [0_f32, 0_f32, 0_f32];
    const spline_coordinates_33: [f32; 4] = [-1_f32, -0.78_f32, -0.5775_f32, -0.375_f32];
    const spline_coordinates_34: [f32; 3] = [0.19999999_f32, 0.44999996_f32, 1_f32];
    const spline_derivatives_34: [f32; 3] = [0_f32, 0_f32, 0_f32];
    const spline_coordinates_35: [f32; 4] = [-1_f32, -0.78_f32, -0.5775_f32, -0.375_f32];
    const spline_derivatives_33: [f32; 4] = [0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_35: [f32; 4] = [0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_36: [f32; 3] = [-0.11_f32, 0.03_f32, 0.65_f32];
    const spline_derivatives_36: [f32; 3] = [0_f32, 0_f32, 0_f32];
    const spline_coordinates_37: [f32; 2] = [-0.2_f32, 0.2_f32];
    const spline_coordinates_38: [f32; 2] = [-0.05_f32, 0.05_f32];
    const spline_coordinates_39: [f32; 2] = [-0.05_f32, 0.05_f32];
    const spline_coordinates_40: [f32; 2] = [0_f32, 0.1_f32];
    const spline_derivatives_40: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_41: [f32; 2] = [-0.9_f32, -0.69_f32];
    const spline_derivatives_37: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_38: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_39: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_41: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_42: [f32; 10] = [-0.6_f32, -0.5_f32, -0.35_f32, -0.25_f32, -0.1_f32, 0.03_f32, 0.35_f32, 0.45_f32, 0.55_f32, 0.62_f32];
    const spline_coordinates_43: [f32; 2] = [-0.2_f32, 0.2_f32];
    const spline_coordinates_44: [f32; 2] = [0_f32, 0.1_f32];
    const spline_derivatives_44: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_45: [f32; 2] = [-0.9_f32, -0.69_f32];
    const spline_derivatives_43: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_45: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_46: [f32; 10] = [-0.6_f32, -0.5_f32, -0.35_f32, -0.25_f32, -0.1_f32, 0.03_f32, 0.35_f32, 0.45_f32, 0.55_f32, 0.62_f32];
    const spline_coordinates_47: [f32; 2] = [-0.2_f32, 0.2_f32];
    const spline_coordinates_48: [f32; 2] = [0_f32, 0.1_f32];
    const spline_derivatives_48: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_49: [f32; 2] = [-0.9_f32, -0.69_f32];
    const spline_derivatives_47: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_49: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_50: [f32; 10] = [-0.6_f32, -0.5_f32, -0.35_f32, -0.25_f32, -0.1_f32, 0.03_f32, 0.35_f32, 0.45_f32, 0.55_f32, 0.62_f32];
    const spline_coordinates_51: [f32; 2] = [-0.2_f32, 0.2_f32];
    const spline_derivatives_51: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_52: [f32; 2] = [0.45_f32, 0.7_f32];
    const spline_coordinates_53: [f32; 2] = [-0.7_f32, -0.15_f32];
    const spline_derivatives_52: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_53: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_54: [f32; 11] = [-0.6_f32, -0.5_f32, -0.35_f32, -0.25_f32, -0.1_f32, 0.03_f32, 0.05_f32, 0.4_f32, 0.45_f32, 0.55_f32, 0.58_f32];
    const spline_derivatives_42: [f32; 10] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_46: [f32; 10] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_50: [f32; 10] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_54: [f32; 11] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_55: [f32; 5] = [-0.19_f32, -0.15_f32, -0.1_f32, 0.03_f32, 0.06_f32];
    const spline_derivatives_55: [f32; 5] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    #[inline]
    fn Beardifier_d16x384x16(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut ret: f64;
        let mut current_piece: Piece;
        let mut current_box: BlockBox;
        let mut current_jigsaw: Jigsaw;
        let mut i: i32;
        let mut m: i32;
        let mut n: i32;
        let mut o: i32;
        let mut p_1: i32;
        let mut q: i32;
        let mut s: i32;
        let mut r: i32;
        let mut l: i32;
        let mut m_1: i32;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        ret = 0_f64;
        if do_beardify(host_consts.base_box, rpos3) {
            i = 0_i32;
            while (i < 500_i32) {
                current_piece = host_consts.piece_array[i as usize];
                current_box = current_piece.block_box;
                if (!valid_box(current_box)) {
                    break;
                }
                m = (max(0_f64, max((current_box.min_x - rpos3.x), (rpos3.x - current_box.max_x))) as i32);
                n = (max(0_f64, max((current_box.min_z - rpos3.z), (rpos3.z - current_box.max_z))) as i32);
                o = ((current_box.min_y + (current_piece.ground_level_delta as f64)) as i32);
                p_1 = ((rpos3.y - (o as f64)) as i32);
                s = current_piece.terrain_adjustment;
                q = 0_i32;
                if ((s == 1_i32) || (s == 2_i32)) {
                    q = p_1;
                }
                if (s == 3_i32) {
                    q = (max(0_f64, max(((o as f64) - rpos3.y), (rpos3.y - current_box.max_y))) as i32);
                }
                if (s == 4_i32) {
                    q = (max(0_f64, max((current_box.min_y - rpos3.y), (rpos3.y - current_box.max_y))) as i32);
                }
                if (s == 1_i32) {
                    ret = (ret + get_magnitude_weight((m as f64), ((q as f64) / 2_f64), (n as f64)));
                }
                if ((s == 2_i32) || (s == 3_i32)) {
                    ret = (ret + (get_structure_weight(m, q, n, p_1) * 0.8_f64));
                }
                if (s == 4_i32) {
                    ret = (ret + (get_magnitude_weight(((m as f64) / 2_f64), ((q as f64) / 2_f64), ((n as f64) / 2_f64)) * 0.8_f64));
                }
                i = (i + 1_i32);
            }
            i = 0_i32;
            while (i < 500_i32) {
                current_jigsaw = host_consts.jigsaw_array[i as usize];
                if (((current_jigsaw.source_x == -1073741824_i32) && (current_jigsaw.source_ground_y == -1073741824_i32)) && (current_jigsaw.source_z == -1073741824_i32)) {
                    break;
                }
                r = ((rpos3.x as i32) - current_jigsaw.source_x);
                l = ((rpos3.y as i32) - current_jigsaw.source_ground_y);
                m_1 = ((rpos3.z as i32) - current_jigsaw.source_z);
                ret = (ret + (get_structure_weight(r, l, m_1, l) * 0.4_f64));
                i = (i + 1_i32);
            }
        }
        return ret;
    }
    
    #[inline]
    fn minecraft_jagged_14_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(1500_f64, 0_f64, 1500_f64)) + (pos3 * Vec3::new(6000_f64, 0_f64, 6000_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.0000152587890625_f64), (&auto_constants.minecraft_jagged_0_octave__16)) * 0.5000076295109483_f64));
        n = (n + (perlin((rpos3 * 0.000015535383426170696_f64), (&auto_constants.minecraft_jagged_1_octave__16)) * 0.5000076295109483_f64));
        n = (n + (perlin((rpos3 * 0.000030517578125_f64), (&auto_constants.minecraft_jagged_0_octave__15)) * 0.2500038147554742_f64));
        n = (n + (perlin((rpos3 * 0.00003107076685234139_f64), (&auto_constants.minecraft_jagged_1_octave__15)) * 0.2500038147554742_f64));
        n = (n + (perlin((rpos3 * 0.00006103515625_f64), (&auto_constants.minecraft_jagged_0_octave__14)) * 0.1250019073777371_f64));
        n = (n + (perlin((rpos3 * 0.00006214153370468278_f64), (&auto_constants.minecraft_jagged_1_octave__14)) * 0.1250019073777371_f64));
        n = (n + (perlin((rpos3 * 0.0001220703125_f64), (&auto_constants.minecraft_jagged_0_octave__13)) * 0.06250095368886854_f64));
        n = (n + (perlin((rpos3 * 0.00012428306740936557_f64), (&auto_constants.minecraft_jagged_1_octave__13)) * 0.06250095368886854_f64));
        n = (n + (perlin((rpos3 * 0.000244140625_f64), (&auto_constants.minecraft_jagged_0_octave__12)) * 0.03125047684443427_f64));
        n = (n + (perlin((rpos3 * 0.00024856613481873114_f64), (&auto_constants.minecraft_jagged_1_octave__12)) * 0.03125047684443427_f64));
        n = (n + (perlin((rpos3 * 0.00048828125_f64), (&auto_constants.minecraft_jagged_0_octave__11)) * 0.015625238422217136_f64));
        n = (n + (perlin((rpos3 * 0.0004971322696374623_f64), (&auto_constants.minecraft_jagged_1_octave__11)) * 0.015625238422217136_f64));
        n = (n + (perlin((rpos3 * 0.0009765625_f64), (&auto_constants.minecraft_jagged_0_octave__10)) * 0.007812619211108568_f64));
        n = (n + (perlin((rpos3 * 0.0009942645392749245_f64), (&auto_constants.minecraft_jagged_1_octave__10)) * 0.007812619211108568_f64));
        n = (n + (perlin((rpos3 * 0.001953125_f64), (&auto_constants.minecraft_jagged_0_octave__9)) * 0.003906309605554284_f64));
        n = (n + (perlin((rpos3 * 0.001988529078549849_f64), (&auto_constants.minecraft_jagged_1_octave__9)) * 0.003906309605554284_f64));
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_jagged_0_octave__8)) * 0.001953154802777142_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_jagged_1_octave__8)) * 0.001953154802777142_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_jagged_0_octave__7)) * 0.000976577401388571_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_jagged_1_octave__7)) * 0.000976577401388571_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_jagged_0_octave__6)) * 0.0004882887006942855_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_jagged_1_octave__6)) * 0.0004882887006942855_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_jagged_0_octave__5)) * 0.00024414435034714275_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_jagged_1_octave__5)) * 0.00024414435034714275_f64));
        n = (n + (perlin((rpos3 * 0.0625_f64), (&auto_constants.minecraft_jagged_0_octave__4)) * 0.00012207217517357137_f64));
        n = (n + (perlin((rpos3 * 0.06363293051359517_f64), (&auto_constants.minecraft_jagged_1_octave__4)) * 0.00012207217517357137_f64));
        n = (n + (perlin((rpos3 * 0.125_f64), (&auto_constants.minecraft_jagged_0_octave__3)) * 0.00006103608758678569_f64));
        n = (n + (perlin((rpos3 * 0.12726586102719034_f64), (&auto_constants.minecraft_jagged_1_octave__3)) * 0.00006103608758678569_f64));
        n = (n + (perlin((rpos3 * 0.25_f64), (&auto_constants.minecraft_jagged_0_octave__2)) * 0.000030518043793392844_f64));
        n = (n + (perlin((rpos3 * 0.2545317220543807_f64), (&auto_constants.minecraft_jagged_1_octave__2)) * 0.000030518043793392844_f64));
        n = (n + (perlin((rpos3 * 0.5_f64), (&auto_constants.minecraft_jagged_0_octave__1)) * 0.000015259021896696422_f64));
        n = (n + (perlin((rpos3 * 0.5090634441087614_f64), (&auto_constants.minecraft_jagged_1_octave__1)) * 0.000015259021896696422_f64));
        return (n * 1.568627450980392_f64);
    }
    
    #[inline]
    fn old_blended_noise_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return base3d_noise(rpos3, (&auto_constants.base3d_perm_table), 8_f64, 80_f64, 0.25_f64, 160_f64, 0.125_f64);
    }
    
    #[inline]
    fn minecraft_cave_layer_24_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(1_f64, 8_f64, 1_f64)) + (pos3 * Vec3::new(4_f64, 64_f64, 4_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_cave_layer_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_cave_layer_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_cave_cheese_25_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(1_f64, 0.6666666666666666_f64, 1_f64)) + (pos3 * Vec3::new(4_f64, 5.333333333333333_f64, 4_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_cave_cheese_0_octave__8)) * 0.25048923679060664_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_cave_cheese_1_octave__8)) * 0.25048923679060664_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_cave_cheese_0_octave__7)) * 0.25048923679060664_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_cave_cheese_1_octave__7)) * 0.25048923679060664_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_cave_cheese_0_octave__6)) * 0.25048923679060664_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_cave_cheese_1_octave__6)) * 0.25048923679060664_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_cave_cheese_0_octave__5)) * 0.06262230919765166_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_cave_cheese_1_octave__5)) * 0.06262230919765166_f64));
        n = (n + (perlin((rpos3 * 0.0625_f64), (&auto_constants.minecraft_cave_cheese_0_octave__4)) * 0.06262230919765166_f64));
        n = (n + (perlin((rpos3 * 0.06363293051359517_f64), (&auto_constants.minecraft_cave_cheese_1_octave__4)) * 0.06262230919765166_f64));
        n = (n + (perlin((rpos3 * 0.125_f64), (&auto_constants.minecraft_cave_cheese_0_octave__3)) * 0.015655577299412915_f64));
        n = (n + (perlin((rpos3 * 0.12726586102719034_f64), (&auto_constants.minecraft_cave_cheese_1_octave__3)) * 0.015655577299412915_f64));
        n = (n + (perlin((rpos3 * 0.5_f64), (&auto_constants.minecraft_cave_cheese_0_octave__1)) * 0.007827788649706457_f64));
        n = (n + (perlin((rpos3 * 0.5090634441087614_f64), (&auto_constants.minecraft_cave_cheese_1_octave__1)) * 0.007827788649706457_f64));
        return (n * 1.4814814814814814_f64);
    }
    
    #[inline]
    fn minecraft_spaghetti_2d_modulator_26_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(2_f64, 1_f64, 2_f64)) + (pos3 * Vec3::new(8_f64, 8_f64, 8_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00048828125_f64), (&auto_constants.minecraft_spaghetti_2d_modulator_0_octave__11)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.0004971322696374623_f64), (&auto_constants.minecraft_spaghetti_2d_modulator_1_octave__11)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_spaghetti_2d_elevation_29_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(1_f64, 0_f64, 1_f64)) + (pos3 * Vec3::new(4_f64, 0_f64, 4_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_spaghetti_2d_elevation_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_spaghetti_2d_elevation_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_noodle_36_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = (origin + (pos3 * Vec3::new(4_f64, 8_f64, 4_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_noodle_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_noodle_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_noodle_thickness_38_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = (origin + (pos3 * Vec3::new(4_f64, 8_f64, 4_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_noodle_thickness_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_noodle_thickness_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_noodle_ridge_a_40_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(2.6666666666666665_f64, 2.6666666666666665_f64, 2.6666666666666665_f64)) + (pos3 * Vec3::new(10.666666666666666_f64, 21.333333333333332_f64, 10.666666666666666_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_noodle_ridge_a_0_octave__7)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_noodle_ridge_a_1_octave__7)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_noodle_ridge_b_42_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(2.6666666666666665_f64, 2.6666666666666665_f64, 2.6666666666666665_f64)) + (pos3 * Vec3::new(10.666666666666666_f64, 21.333333333333332_f64, 10.666666666666666_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_noodle_ridge_b_0_octave__7)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_noodle_ridge_b_1_octave__7)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_ore_vein_a_49_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(4_f64, 4_f64, 4_f64)) + (pos3 * Vec3::new(16_f64, 32_f64, 16_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_ore_vein_a_0_octave__7)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_ore_vein_a_1_octave__7)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_ore_vein_b_51_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(4_f64, 4_f64, 4_f64)) + (pos3 * Vec3::new(16_f64, 32_f64, 16_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_ore_vein_b_0_octave__7)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_ore_vein_b_1_octave__7)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_ore_veininess_53_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(1.5_f64, 1.5_f64, 1.5_f64)) + (pos3 * Vec3::new(6_f64, 12_f64, 6_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_ore_veininess_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_ore_veininess_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_cave_entrance_17_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.75_f64, 0.5_f64, 0.75_f64)) + (pos3 * Vec3::new(3_f64, 4_f64, 3_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_cave_entrance_0_octave__7)) * 0.22857142857142856_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_cave_entrance_1_octave__7)) * 0.22857142857142856_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_cave_entrance_0_octave__6)) * 0.14285714285714285_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_cave_entrance_1_octave__6)) * 0.14285714285714285_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_cave_entrance_0_octave__5)) * 0.14285714285714285_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_cave_entrance_1_octave__5)) * 0.14285714285714285_f64));
        return (n * 1.25_f64);
    }
    
    #[inline]
    fn minecraft_spaghetti_3d_thickness_23_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = (origin + (pos3 * Vec3::new(4_f64, 8_f64, 4_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_spaghetti_3d_thickness_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_spaghetti_3d_thickness_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_spaghetti_2d_thickness_28_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(2_f64, 1_f64, 2_f64)) + (pos3 * Vec3::new(8_f64, 8_f64, 8_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00048828125_f64), (&auto_constants.minecraft_spaghetti_2d_thickness_0_octave__11)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.0004971322696374623_f64), (&auto_constants.minecraft_spaghetti_2d_thickness_1_octave__11)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_spaghetti_roughness_modulator_19_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = (origin + (pos3 * Vec3::new(4_f64, 8_f64, 4_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_spaghetti_roughness_modulator_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_spaghetti_roughness_modulator_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_spaghetti_roughness_20_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = (origin + (pos3 * Vec3::new(4_f64, 8_f64, 4_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_spaghetti_roughness_0_octave__5)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_spaghetti_roughness_1_octave__5)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_pillar_32_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(25_f64, 0.3_f64, 25_f64)) + (pos3 * Vec3::new(100_f64, 2.4_f64, 100_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_pillar_0_octave__7)) * 0.6666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_pillar_1_octave__7)) * 0.6666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_pillar_0_octave__6)) * 0.3333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_pillar_1_octave__6)) * 0.3333333333333333_f64));
        return (n * 1.111111111111111_f64);
    }
    
    #[inline]
    fn minecraft_pillar_rareness_33_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = (origin + (pos3 * Vec3::new(4_f64, 8_f64, 4_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_pillar_rareness_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_pillar_rareness_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_pillar_thickness_34_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = (origin + (pos3 * Vec3::new(4_f64, 8_f64, 4_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_pillar_thickness_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_pillar_thickness_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_spaghetti_3d_rarity_22_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(2_f64, 1_f64, 2_f64)) + (pos3 * Vec3::new(8_f64, 8_f64, 8_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00048828125_f64), (&auto_constants.minecraft_spaghetti_3d_rarity_0_octave__11)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.0004971322696374623_f64), (&auto_constants.minecraft_spaghetti_3d_rarity_1_octave__11)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_offset_3_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.25_f64, 0_f64, 0.25_f64)) + (pos3 * Vec3::new(1_f64, 0_f64, 1_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.125_f64), (&auto_constants.minecraft_offset_0_octave__3)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.12726586102719034_f64), (&auto_constants.minecraft_offset_1_octave__3)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.25_f64), (&auto_constants.minecraft_offset_0_octave__2)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.2545317220543807_f64), (&auto_constants.minecraft_offset_1_octave__2)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.5_f64), (&auto_constants.minecraft_offset_0_octave__1)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.5090634441087614_f64), (&auto_constants.minecraft_offset_1_octave__1)) * 0.13333333333333333_f64));
        return (n * 1.25_f64);
    }
    
    #[inline]
    fn minecraft_offset_3_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.25_f64, 0_f64, 0.25_f64)) + (pos3 * Vec3::new(1_f64, 0_f64, 1_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.125_f64), (&auto_constants.minecraft_offset_0_octave__3)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.12726586102719034_f64), (&auto_constants.minecraft_offset_1_octave__3)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.25_f64), (&auto_constants.minecraft_offset_0_octave__2)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.2545317220543807_f64), (&auto_constants.minecraft_offset_1_octave__2)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.5_f64), (&auto_constants.minecraft_offset_0_octave__1)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.5090634441087614_f64), (&auto_constants.minecraft_offset_1_octave__1)) * 0.13333333333333333_f64));
        return (n * 1.25_f64);
    }
    
    #[inline]
    fn density_function_RangeChoice_35_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut var_1: f64;
        let mut var_2: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = clamp(((rpos3.y - -4064_f64) / 8126_f64), 0_f64, 1_f64);
        var_1 = ((var * 8126_f64) + -4064_f64);
        if ((var_1 >= -60_f64) && (var_1 < 321_f64)) {
            var_2 = buffers.minecraft_noodle_36_d5x49x5_output[index(p) as usize];
        } else {
            var_2 = -1_f64;
        }
        return var_2;
    }
    
    #[inline]
    fn density_function_RangeChoice_37_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut var_1: f64;
        let mut var_2: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = clamp(((rpos3.y - -4064_f64) / 8126_f64), 0_f64, 1_f64);
        var_1 = ((var * 8126_f64) + -4064_f64);
        if ((var_1 >= -60_f64) && (var_1 < 321_f64)) {
            var_2 = (-0.07500000000000001_f64 + (-0.025_f64 * buffers.minecraft_noodle_thickness_38_d5x49x5_output[index(p) as usize]));
        } else {
            var_2 = 0_f64;
        }
        return var_2;
    }
    
    #[inline]
    fn density_function_RangeChoice_39_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut var_1: f64;
        let mut var_2: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = clamp(((rpos3.y - -4064_f64) / 8126_f64), 0_f64, 1_f64);
        var_1 = ((var * 8126_f64) + -4064_f64);
        if ((var_1 >= -60_f64) && (var_1 < 321_f64)) {
            var_2 = buffers.minecraft_noodle_ridge_a_40_d5x49x5_output[index(p) as usize];
        } else {
            var_2 = 0_f64;
        }
        return var_2;
    }
    
    #[inline]
    fn density_function_RangeChoice_41_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut var_1: f64;
        let mut var_2: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = clamp(((rpos3.y - -4064_f64) / 8126_f64), 0_f64, 1_f64);
        var_1 = ((var * 8126_f64) + -4064_f64);
        if ((var_1 >= -60_f64) && (var_1 < 321_f64)) {
            var_2 = buffers.minecraft_noodle_ridge_b_42_d5x49x5_output[index(p) as usize];
        } else {
            var_2 = 0_f64;
        }
        return var_2;
    }
    
    #[inline]
    fn density_function_RangeChoice_48_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut var_1: f64;
        let mut var_2: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = clamp(((rpos3.y - -4064_f64) / 8126_f64), 0_f64, 1_f64);
        var_1 = ((var * 8126_f64) + -4064_f64);
        if ((var_1 >= -60_f64) && (var_1 < 51_f64)) {
            var_2 = buffers.minecraft_ore_vein_a_49_d5x49x5_output[index(p) as usize];
        } else {
            var_2 = 0_f64;
        }
        return var_2;
    }
    
    #[inline]
    fn density_function_RangeChoice_50_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut var_1: f64;
        let mut var_2: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = clamp(((rpos3.y - -4064_f64) / 8126_f64), 0_f64, 1_f64);
        var_1 = ((var * 8126_f64) + -4064_f64);
        if ((var_1 >= -60_f64) && (var_1 < 51_f64)) {
            var_2 = buffers.minecraft_ore_vein_b_51_d5x49x5_output[index(p) as usize];
        } else {
            var_2 = 0_f64;
        }
        return var_2;
    }
    
    #[inline]
    fn density_function_RangeChoice_52_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut var_1: f64;
        let mut var_2: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = clamp(((rpos3.y - -4064_f64) / 8126_f64), 0_f64, 1_f64);
        var_1 = ((var * 8126_f64) + -4064_f64);
        if ((var_1 >= -60_f64) && (var_1 < 51_f64)) {
            var_2 = buffers.minecraft_ore_veininess_53_d5x49x5_output[index(p) as usize];
        } else {
            var_2 = 0_f64;
        }
        return var_2;
    }
    
    #[inline]
    fn density_function_Add_27_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return (-0.95_f64 + (-0.35000000000000003_f64 * buffers.minecraft_spaghetti_2d_thickness_28_d5x49x5_output[index(p) as usize]));
    }
    
    #[inline]
    fn density_function_Multiply_18_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return ((-0.05_f64 + (-0.05_f64 * buffers.minecraft_spaghetti_roughness_modulator_19_d5x49x5_output[index(p) as usize])) * (-0.4_f64 + abs(buffers.minecraft_spaghetti_roughness_20_d5x49x5_output[index(p) as usize])));
    }
    
    #[inline]
    fn density_function_Multiply_31_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = (0.55_f64 + (0.55_f64 * buffers.minecraft_pillar_thickness_34_d5x49x5_output[index(p) as usize]));
        return (((2_f64 * buffers.minecraft_pillar_32_d5x49x5_output[index(p) as usize]) + (-1_f64 + (-1_f64 * buffers.minecraft_pillar_rareness_33_d5x49x5_output[index(p) as usize]))) * (var * (var * var)));
    }
    
    #[inline]
    fn density_function_Noise_21_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_spaghetti_3d_rarity_22_d5x49x5_output[index(p) as usize];
    }
    
    #[inline]
    fn density_function_ShiftedNoise_55_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return minecraft_ridge_shifted_57(p, config, buffers, auto_constants, host_consts, ((rpos3 * Vec3::new(0.25_f64, 0_f64, 0.25_f64)) + Vec3::new((buffers.minecraft_offset_3_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 1_i32) as usize] * 4_f64), 0_f64, (minecraft_offset_shiftb_56(p, config, buffers, auto_constants, host_consts, Vec3::new((rpos3.z / 4_f64), (rpos3.x / 4_f64), 0_f64)) * 4_f64))));
    }
    
    #[inline]
    fn minecraft_ridge_shifted_57(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_ridge_0_octave__7)) * 0.5079365079365079_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_ridge_1_octave__7)) * 0.5079365079365079_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_ridge_0_octave__6)) * 0.5079365079365079_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_ridge_1_octave__6)) * 0.5079365079365079_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_ridge_0_octave__5)) * 0.12698412698412698_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_ridge_1_octave__5)) * 0.12698412698412698_f64));
        return (n * 1.25_f64);
    }
    
    #[inline]
    fn minecraft_offset_shiftb_56(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.125_f64), (&auto_constants.minecraft_offset_0_octave__3)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.12726586102719034_f64), (&auto_constants.minecraft_offset_1_octave__3)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.25_f64), (&auto_constants.minecraft_offset_0_octave__2)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.2545317220543807_f64), (&auto_constants.minecraft_offset_1_octave__2)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.5_f64), (&auto_constants.minecraft_offset_0_octave__1)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.5090634441087614_f64), (&auto_constants.minecraft_offset_1_octave__1)) * 0.13333333333333333_f64));
        return (n * 1.25_f64);
    }
    
    #[inline]
    fn density_function_ShiftedNoise_58_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return minecraft_erosion_shifted_60(p, config, buffers, auto_constants, host_consts, ((rpos3 * Vec3::new(0.25_f64, 0_f64, 0.25_f64)) + Vec3::new((buffers.minecraft_offset_3_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 1_i32) as usize] * 4_f64), 0_f64, (minecraft_offset_shiftb_59(p, config, buffers, auto_constants, host_consts, Vec3::new((rpos3.z / 4_f64), (rpos3.x / 4_f64), 0_f64)) * 4_f64))));
    }
    
    #[inline]
    fn minecraft_erosion_shifted_60(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.001953125_f64), (&auto_constants.minecraft_erosion_0_octave__9)) * 0.5161290322580645_f64));
        n = (n + (perlin((rpos3 * 0.001988529078549849_f64), (&auto_constants.minecraft_erosion_1_octave__9)) * 0.5161290322580645_f64));
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_erosion_0_octave__8)) * 0.25806451612903225_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_erosion_1_octave__8)) * 0.25806451612903225_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_erosion_0_octave__6)) * 0.06451612903225806_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_erosion_1_octave__6)) * 0.06451612903225806_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_erosion_0_octave__5)) * 0.03225806451612903_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_erosion_1_octave__5)) * 0.03225806451612903_f64));
        return (n * 1.3888888888888888_f64);
    }
    
    #[inline]
    fn minecraft_offset_shiftb_59(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.125_f64), (&auto_constants.minecraft_offset_0_octave__3)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.12726586102719034_f64), (&auto_constants.minecraft_offset_1_octave__3)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.25_f64), (&auto_constants.minecraft_offset_0_octave__2)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.2545317220543807_f64), (&auto_constants.minecraft_offset_1_octave__2)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.5_f64), (&auto_constants.minecraft_offset_0_octave__1)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.5090634441087614_f64), (&auto_constants.minecraft_offset_1_octave__1)) * 0.13333333333333333_f64));
        return (n * 1.25_f64);
    }
    
    #[inline]
    fn density_function_ShiftedNoise_61_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return minecraft_continentalness_shifted_63(p, config, buffers, auto_constants, host_consts, ((rpos3 * Vec3::new(0.25_f64, 0_f64, 0.25_f64)) + Vec3::new((buffers.minecraft_offset_3_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 1_i32) as usize] * 4_f64), 0_f64, (minecraft_offset_shiftb_62(p, config, buffers, auto_constants, host_consts, Vec3::new((rpos3.z / 4_f64), (rpos3.x / 4_f64), 0_f64)) * 4_f64))));
    }
    
    #[inline]
    fn minecraft_continentalness_shifted_63(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.001953125_f64), (&auto_constants.minecraft_continentalness_0_octave__9)) * 0.5009784735812133_f64));
        n = (n + (perlin((rpos3 * 0.001988529078549849_f64), (&auto_constants.minecraft_continentalness_1_octave__9)) * 0.5009784735812133_f64));
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_continentalness_0_octave__8)) * 0.25048923679060664_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_continentalness_1_octave__8)) * 0.25048923679060664_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_continentalness_0_octave__7)) * 0.25048923679060664_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_continentalness_1_octave__7)) * 0.25048923679060664_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_continentalness_0_octave__6)) * 0.12524461839530332_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_continentalness_1_octave__6)) * 0.12524461839530332_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_continentalness_0_octave__5)) * 0.06262230919765166_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_continentalness_1_octave__5)) * 0.06262230919765166_f64));
        n = (n + (perlin((rpos3 * 0.0625_f64), (&auto_constants.minecraft_continentalness_0_octave__4)) * 0.015655577299412915_f64));
        n = (n + (perlin((rpos3 * 0.06363293051359517_f64), (&auto_constants.minecraft_continentalness_1_octave__4)) * 0.015655577299412915_f64));
        n = (n + (perlin((rpos3 * 0.125_f64), (&auto_constants.minecraft_continentalness_0_octave__3)) * 0.007827788649706457_f64));
        n = (n + (perlin((rpos3 * 0.12726586102719034_f64), (&auto_constants.minecraft_continentalness_1_octave__3)) * 0.007827788649706457_f64));
        n = (n + (perlin((rpos3 * 0.25_f64), (&auto_constants.minecraft_continentalness_0_octave__2)) * 0.003913894324853229_f64));
        n = (n + (perlin((rpos3 * 0.2545317220543807_f64), (&auto_constants.minecraft_continentalness_1_octave__2)) * 0.003913894324853229_f64));
        n = (n + (perlin((rpos3 * 0.5_f64), (&auto_constants.minecraft_continentalness_0_octave__1)) * 0.0019569471624266144_f64));
        n = (n + (perlin((rpos3 * 0.5090634441087614_f64), (&auto_constants.minecraft_continentalness_1_octave__1)) * 0.0019569471624266144_f64));
        return (n * 1.4999999999999998_f64);
    }
    
    #[inline]
    fn minecraft_offset_shiftb_62(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.125_f64), (&auto_constants.minecraft_offset_0_octave__3)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.12726586102719034_f64), (&auto_constants.minecraft_offset_1_octave__3)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.25_f64), (&auto_constants.minecraft_offset_0_octave__2)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.2545317220543807_f64), (&auto_constants.minecraft_offset_1_octave__2)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.5_f64), (&auto_constants.minecraft_offset_0_octave__1)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.5090634441087614_f64), (&auto_constants.minecraft_offset_1_octave__1)) * 0.13333333333333333_f64));
        return (n * 1.25_f64);
    }
    
    #[inline]
    fn density_function_ShiftedNoise_2_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return minecraft_ridge_shifted_5(p, config, buffers, auto_constants, host_consts, ((rpos3 * Vec3::new(0.25_f64, 0_f64, 0.25_f64)) + Vec3::new((buffers.minecraft_offset_3_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 1_i32) as usize] * 4_f64), 0_f64, (minecraft_offset_shiftb_4(p, config, buffers, auto_constants, host_consts, Vec3::new((rpos3.z / 4_f64), (rpos3.x / 4_f64), 0_f64)) * 4_f64))));
    }
    
    #[inline]
    fn minecraft_ridge_shifted_5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_ridge_0_octave__7)) * 0.5079365079365079_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_ridge_1_octave__7)) * 0.5079365079365079_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_ridge_0_octave__6)) * 0.5079365079365079_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_ridge_1_octave__6)) * 0.5079365079365079_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_ridge_0_octave__5)) * 0.12698412698412698_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_ridge_1_octave__5)) * 0.12698412698412698_f64));
        return (n * 1.25_f64);
    }
    
    #[inline]
    fn minecraft_offset_shiftb_4(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.125_f64), (&auto_constants.minecraft_offset_0_octave__3)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.12726586102719034_f64), (&auto_constants.minecraft_offset_1_octave__3)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.25_f64), (&auto_constants.minecraft_offset_0_octave__2)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.2545317220543807_f64), (&auto_constants.minecraft_offset_1_octave__2)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.5_f64), (&auto_constants.minecraft_offset_0_octave__1)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.5090634441087614_f64), (&auto_constants.minecraft_offset_1_octave__1)) * 0.13333333333333333_f64));
        return (n * 1.25_f64);
    }
    
    #[inline]
    fn density_function_ShiftedNoise_6_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return minecraft_erosion_shifted_8(p, config, buffers, auto_constants, host_consts, ((rpos3 * Vec3::new(0.25_f64, 0_f64, 0.25_f64)) + Vec3::new((buffers.minecraft_offset_3_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 1_i32) as usize] * 4_f64), 0_f64, (minecraft_offset_shiftb_7(p, config, buffers, auto_constants, host_consts, Vec3::new((rpos3.z / 4_f64), (rpos3.x / 4_f64), 0_f64)) * 4_f64))));
    }
    
    #[inline]
    fn minecraft_erosion_shifted_8(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.001953125_f64), (&auto_constants.minecraft_erosion_0_octave__9)) * 0.5161290322580645_f64));
        n = (n + (perlin((rpos3 * 0.001988529078549849_f64), (&auto_constants.minecraft_erosion_1_octave__9)) * 0.5161290322580645_f64));
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_erosion_0_octave__8)) * 0.25806451612903225_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_erosion_1_octave__8)) * 0.25806451612903225_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_erosion_0_octave__6)) * 0.06451612903225806_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_erosion_1_octave__6)) * 0.06451612903225806_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_erosion_0_octave__5)) * 0.03225806451612903_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_erosion_1_octave__5)) * 0.03225806451612903_f64));
        return (n * 1.3888888888888888_f64);
    }
    
    #[inline]
    fn minecraft_offset_shiftb_7(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.125_f64), (&auto_constants.minecraft_offset_0_octave__3)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.12726586102719034_f64), (&auto_constants.minecraft_offset_1_octave__3)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.25_f64), (&auto_constants.minecraft_offset_0_octave__2)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.2545317220543807_f64), (&auto_constants.minecraft_offset_1_octave__2)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.5_f64), (&auto_constants.minecraft_offset_0_octave__1)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.5090634441087614_f64), (&auto_constants.minecraft_offset_1_octave__1)) * 0.13333333333333333_f64));
        return (n * 1.25_f64);
    }
    
    #[inline]
    fn density_function_ShiftedNoise_9_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return minecraft_continentalness_shifted_11(p, config, buffers, auto_constants, host_consts, ((rpos3 * Vec3::new(0.25_f64, 0_f64, 0.25_f64)) + Vec3::new((buffers.minecraft_offset_3_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 1_i32) as usize] * 4_f64), 0_f64, (minecraft_offset_shiftb_10(p, config, buffers, auto_constants, host_consts, Vec3::new((rpos3.z / 4_f64), (rpos3.x / 4_f64), 0_f64)) * 4_f64))));
    }
    
    #[inline]
    fn minecraft_continentalness_shifted_11(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.001953125_f64), (&auto_constants.minecraft_continentalness_0_octave__9)) * 0.5009784735812133_f64));
        n = (n + (perlin((rpos3 * 0.001988529078549849_f64), (&auto_constants.minecraft_continentalness_1_octave__9)) * 0.5009784735812133_f64));
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_continentalness_0_octave__8)) * 0.25048923679060664_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_continentalness_1_octave__8)) * 0.25048923679060664_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_continentalness_0_octave__7)) * 0.25048923679060664_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_continentalness_1_octave__7)) * 0.25048923679060664_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_continentalness_0_octave__6)) * 0.12524461839530332_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_continentalness_1_octave__6)) * 0.12524461839530332_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_continentalness_0_octave__5)) * 0.06262230919765166_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_continentalness_1_octave__5)) * 0.06262230919765166_f64));
        n = (n + (perlin((rpos3 * 0.0625_f64), (&auto_constants.minecraft_continentalness_0_octave__4)) * 0.015655577299412915_f64));
        n = (n + (perlin((rpos3 * 0.06363293051359517_f64), (&auto_constants.minecraft_continentalness_1_octave__4)) * 0.015655577299412915_f64));
        n = (n + (perlin((rpos3 * 0.125_f64), (&auto_constants.minecraft_continentalness_0_octave__3)) * 0.007827788649706457_f64));
        n = (n + (perlin((rpos3 * 0.12726586102719034_f64), (&auto_constants.minecraft_continentalness_1_octave__3)) * 0.007827788649706457_f64));
        n = (n + (perlin((rpos3 * 0.25_f64), (&auto_constants.minecraft_continentalness_0_octave__2)) * 0.003913894324853229_f64));
        n = (n + (perlin((rpos3 * 0.2545317220543807_f64), (&auto_constants.minecraft_continentalness_1_octave__2)) * 0.003913894324853229_f64));
        n = (n + (perlin((rpos3 * 0.5_f64), (&auto_constants.minecraft_continentalness_0_octave__1)) * 0.0019569471624266144_f64));
        n = (n + (perlin((rpos3 * 0.5090634441087614_f64), (&auto_constants.minecraft_continentalness_1_octave__1)) * 0.0019569471624266144_f64));
        return (n * 1.4999999999999998_f64);
    }
    
    #[inline]
    fn minecraft_offset_shiftb_10(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.125_f64), (&auto_constants.minecraft_offset_0_octave__3)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.12726586102719034_f64), (&auto_constants.minecraft_offset_1_octave__3)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.25_f64), (&auto_constants.minecraft_offset_0_octave__2)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.2545317220543807_f64), (&auto_constants.minecraft_offset_1_octave__2)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.5_f64), (&auto_constants.minecraft_offset_0_octave__1)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.5090634441087614_f64), (&auto_constants.minecraft_offset_1_octave__1)) * 0.13333333333333333_f64));
        return (n * 1.25_f64);
    }
    
    #[inline]
    fn vein_ridged_d16x384x16(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return (-0.07999999821186066_f64 + max(abs(interpolate(buffers.density_function_RangeChoice_48_d5x49x5_output[cornerx0y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_48_d5x49x5_output[cornerx4y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_48_d5x49x5_output[cornerx0y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_48_d5x49x5_output[cornerx4y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_48_d5x49x5_output[cornerx0y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_48_d5x49x5_output[cornerx4y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_48_d5x49x5_output[cornerx0y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_48_d5x49x5_output[cornerx4y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], xfract4(pos3), yfract8(pos3), zfract4(pos3))), abs(interpolate(buffers.density_function_RangeChoice_50_d5x49x5_output[cornerx0y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_50_d5x49x5_output[cornerx4y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_50_d5x49x5_output[cornerx0y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_50_d5x49x5_output[cornerx4y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_50_d5x49x5_output[cornerx0y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_50_d5x49x5_output[cornerx4y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_50_d5x49x5_output[cornerx0y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_50_d5x49x5_output[cornerx4y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], xfract4(pos3), yfract8(pos3), zfract4(pos3)))));
    }
    
    #[inline]
    fn vein_toggle_d16x384x16(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return interpolate(buffers.density_function_RangeChoice_52_d5x49x5_output[cornerx0y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_52_d5x49x5_output[cornerx4y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_52_d5x49x5_output[cornerx0y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_52_d5x49x5_output[cornerx4y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_52_d5x49x5_output[cornerx0y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_52_d5x49x5_output[cornerx4y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_52_d5x49x5_output[cornerx0y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_52_d5x49x5_output[cornerx4y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], xfract4(pos3), yfract8(pos3), zfract4(pos3));
    }
    
    #[inline]
    fn density_function_Min_16_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut var_1: f64;
        let mut var_2: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = clamp(((rpos3.y - -10_f64) / 40_f64), 0_f64, 1_f64);
        var_1 = scale_tunnels(buffers.density_function_Noise_21_d5x49x5_output[index(p) as usize]);
        var_2 = scale_tunnels(buffers.density_function_Noise_21_d5x49x5_output[index(p) as usize]);
        return min(((0.37_f64 + buffers.minecraft_cave_entrance_17_d5x49x5_output[index(p) as usize]) + ((var * -0.3_f64) + 0.3_f64)), (buffers.density_function_Multiply_18_d5x49x5_output[index(p) as usize] + clamp((max((var_1 * abs(minecraft_spaghetti_3d_1_rarity_mapped_1(p, config, buffers, auto_constants, host_consts, (rpos3 / var_1)))), (var_2 * abs(minecraft_spaghetti_3d_2_rarity_mapped_1(p, config, buffers, auto_constants, host_consts, (rpos3 / var_2))))) + (-0.0765_f64 + (-0.011499999999999996_f64 * buffers.minecraft_spaghetti_3d_thickness_23_d5x49x5_output[index(p) as usize]))), -1_f64, 1_f64)));
    }
    
    #[inline]
    fn minecraft_spaghetti_3d_1_rarity_mapped_1(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_spaghetti_3d_1_0_octave__7)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_spaghetti_3d_1_1_octave__7)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_spaghetti_3d_2_rarity_mapped_1(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_spaghetti_3d_2_0_octave__7)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_spaghetti_3d_2_1_octave__7)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_overworld_ridges_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.density_function_ShiftedNoise_55_d128x1x128_output[index(p) as usize];
    }
    
    #[inline]
    fn minecraft_overworld_ridges_folded_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return (-3_f64 * (-0.3333333333333333_f64 + abs((-0.6666666666666666_f64 + abs(buffers.density_function_ShiftedNoise_55_d128x1x128_output[index(p) as usize])))));
    }
    
    #[inline]
    fn minecraft_overworld_erosion_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.density_function_ShiftedNoise_58_d128x1x128_output[index(p) as usize];
    }
    
    #[inline]
    fn minecraft_overworld_continents_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.density_function_ShiftedNoise_61_d128x1x128_output[index(p) as usize];
    }
    
    #[inline]
    fn minecraft_overworld_ridges_folded_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return (-3_f64 * (-0.3333333333333333_f64 + abs((-0.6666666666666666_f64 + abs(buffers.density_function_ShiftedNoise_2_d5x1x5_output[index(p) as usize])))));
    }
    
    #[inline]
    fn minecraft_overworld_erosion_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.density_function_ShiftedNoise_6_d5x1x5_output[index(p) as usize];
    }
    
    #[inline]
    fn minecraft_overworld_continents_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.density_function_ShiftedNoise_9_d5x1x5_output[index(p) as usize];
    }
    
    #[inline]
    fn minecraft_overworld_ridges_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.density_function_ShiftedNoise_2_d5x1x5_output[index(p) as usize];
    }
    
    #[inline]
    fn density_function_Cache2d_64_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 2];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_values_1: [f32; 2];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 2];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 2];
        let mut coordinate_3: f32;
        let mut spline_index_3: i32;
        let mut spline_values_4: [f32; 2];
        let mut coordinate_4: f32;
        let mut spline_index_4: i32;
        let mut spline_values_5: [f32; 10];
        let mut coordinate_5: f32;
        let mut spline_index_5: i32;
        let mut spline_values_6: [f32; 2];
        let mut coordinate_6: f32;
        let mut spline_index_6: i32;
        let mut spline_values_7: [f32; 2];
        let mut coordinate_7: f32;
        let mut spline_index_7: i32;
        let mut spline_values_8: [f32; 2];
        let mut coordinate_8: f32;
        let mut spline_index_8: i32;
        let mut spline_values_9: [f32; 10];
        let mut coordinate_9: f32;
        let mut spline_index_9: i32;
        let mut spline_values_10: [f32; 2];
        let mut coordinate_10: f32;
        let mut spline_index_10: i32;
        let mut spline_values_11: [f32; 2];
        let mut coordinate_11: f32;
        let mut spline_index_11: i32;
        let mut spline_values_12: [f32; 2];
        let mut coordinate_12: f32;
        let mut spline_index_12: i32;
        let mut spline_values_13: [f32; 10];
        let mut coordinate_13: f32;
        let mut spline_index_13: i32;
        let mut spline_values_14: [f32; 2];
        let mut coordinate_14: f32;
        let mut spline_index_14: i32;
        let mut spline_values_15: [f32; 2];
        let mut coordinate_15: f32;
        let mut spline_index_15: i32;
        let mut spline_values_16: [f32; 2];
        let mut coordinate_16: f32;
        let mut spline_index_16: i32;
        let mut spline_values_17: [f32; 11];
        let mut coordinate_17: f32;
        let mut spline_index_17: i32;
        let mut spline_values_18: [f32; 5];
        let mut coordinate_18: f32;
        let mut spline_index_18: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [6.3_f32, 6.25_f32];
        coordinate = (buffers.minecraft_overworld_ridges_d128x1x128_output[index(p) as usize] as f32);
        spline_index = binary_search(spline_coordinates_85, (coordinate as f32));
        spline_values_1 = [6.3_f32, 2.67_f32];
        coordinate_1 = (buffers.minecraft_overworld_ridges_d128x1x128_output[index(p) as usize] as f32);
        spline_index_1 = binary_search(spline_coordinates_86, (coordinate_1 as f32));
        spline_values_2 = [2.67_f32, 6.3_f32];
        coordinate_2 = (buffers.minecraft_overworld_ridges_d128x1x128_output[index(p) as usize] as f32);
        spline_index_2 = binary_search(spline_coordinates_87, (coordinate_2 as f32));
        spline_values_3 = [6.25_f32, 0.625_f32];
        coordinate_3 = (buffers.minecraft_overworld_ridges_d128x1x128_output[index(p) as usize] as f32);
        spline_index_3 = binary_search(spline_coordinates_88, (coordinate_3 as f32));
        spline_values_4 = [6.25_f32, advanced_hermite(spline_coordinates_88, spline_values_3, spline_derivatives_88, (coordinate_3 as f32), (spline_index_3 as i32))];
        coordinate_4 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_4 = binary_search(spline_coordinates_89, (coordinate_4 as f32));
        spline_values_5 = [advanced_hermite(spline_coordinates_85, spline_values, spline_derivatives_85, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_86, spline_values_1, spline_derivatives_86, (coordinate_1 as f32), (spline_index_1 as i32)), advanced_hermite(spline_coordinates_85, spline_values, spline_derivatives_85, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_85, spline_values, spline_derivatives_85, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_87, spline_values_2, spline_derivatives_87, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_85, spline_values, spline_derivatives_85, (coordinate as f32), (spline_index as i32)), 6.25_f32, advanced_hermite(spline_coordinates_89, spline_values_4, spline_derivatives_89, (coordinate_4 as f32), (spline_index_4 as i32)), advanced_hermite(spline_coordinates_89, spline_values_4, spline_derivatives_89, (coordinate_4 as f32), (spline_index_4 as i32)), 6.25_f32];
        coordinate_5 = (buffers.minecraft_overworld_erosion_d128x1x128_output[index(p) as usize] as f32);
        spline_index_5 = binary_search(spline_coordinates_90, (coordinate_5 as f32));
        spline_values_6 = [6.3_f32, 5.47_f32];
        coordinate_6 = (buffers.minecraft_overworld_ridges_d128x1x128_output[index(p) as usize] as f32);
        spline_index_6 = binary_search(spline_coordinates_91, (coordinate_6 as f32));
        spline_values_7 = [5.47_f32, 0.625_f32];
        coordinate_7 = (buffers.minecraft_overworld_ridges_d128x1x128_output[index(p) as usize] as f32);
        spline_index_7 = binary_search(spline_coordinates_92, (coordinate_7 as f32));
        spline_values_8 = [5.47_f32, advanced_hermite(spline_coordinates_92, spline_values_7, spline_derivatives_92, (coordinate_7 as f32), (spline_index_7 as i32))];
        coordinate_8 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_8 = binary_search(spline_coordinates_93, (coordinate_8 as f32));
        spline_values_9 = [advanced_hermite(spline_coordinates_91, spline_values_6, spline_derivatives_91, (coordinate_6 as f32), (spline_index_6 as i32)), advanced_hermite(spline_coordinates_86, spline_values_1, spline_derivatives_86, (coordinate_1 as f32), (spline_index_1 as i32)), advanced_hermite(spline_coordinates_91, spline_values_6, spline_derivatives_91, (coordinate_6 as f32), (spline_index_6 as i32)), advanced_hermite(spline_coordinates_91, spline_values_6, spline_derivatives_91, (coordinate_6 as f32), (spline_index_6 as i32)), advanced_hermite(spline_coordinates_87, spline_values_2, spline_derivatives_87, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_91, spline_values_6, spline_derivatives_91, (coordinate_6 as f32), (spline_index_6 as i32)), 5.47_f32, advanced_hermite(spline_coordinates_93, spline_values_8, spline_derivatives_93, (coordinate_8 as f32), (spline_index_8 as i32)), advanced_hermite(spline_coordinates_93, spline_values_8, spline_derivatives_93, (coordinate_8 as f32), (spline_index_8 as i32)), 5.47_f32];
        coordinate_9 = (buffers.minecraft_overworld_erosion_d128x1x128_output[index(p) as usize] as f32);
        spline_index_9 = binary_search(spline_coordinates_94, (coordinate_9 as f32));
        spline_values_10 = [6.3_f32, 5.08_f32];
        coordinate_10 = (buffers.minecraft_overworld_ridges_d128x1x128_output[index(p) as usize] as f32);
        spline_index_10 = binary_search(spline_coordinates_95, (coordinate_10 as f32));
        spline_values_11 = [5.08_f32, 0.625_f32];
        coordinate_11 = (buffers.minecraft_overworld_ridges_d128x1x128_output[index(p) as usize] as f32);
        spline_index_11 = binary_search(spline_coordinates_96, (coordinate_11 as f32));
        spline_values_12 = [5.08_f32, advanced_hermite(spline_coordinates_96, spline_values_11, spline_derivatives_96, (coordinate_11 as f32), (spline_index_11 as i32))];
        coordinate_12 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_12 = binary_search(spline_coordinates_97, (coordinate_12 as f32));
        spline_values_13 = [advanced_hermite(spline_coordinates_95, spline_values_10, spline_derivatives_95, (coordinate_10 as f32), (spline_index_10 as i32)), advanced_hermite(spline_coordinates_86, spline_values_1, spline_derivatives_86, (coordinate_1 as f32), (spline_index_1 as i32)), advanced_hermite(spline_coordinates_95, spline_values_10, spline_derivatives_95, (coordinate_10 as f32), (spline_index_10 as i32)), advanced_hermite(spline_coordinates_95, spline_values_10, spline_derivatives_95, (coordinate_10 as f32), (spline_index_10 as i32)), advanced_hermite(spline_coordinates_87, spline_values_2, spline_derivatives_87, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_95, spline_values_10, spline_derivatives_95, (coordinate_10 as f32), (spline_index_10 as i32)), 5.08_f32, advanced_hermite(spline_coordinates_97, spline_values_12, spline_derivatives_97, (coordinate_12 as f32), (spline_index_12 as i32)), advanced_hermite(spline_coordinates_97, spline_values_12, spline_derivatives_97, (coordinate_12 as f32), (spline_index_12 as i32)), 5.08_f32];
        coordinate_13 = (buffers.minecraft_overworld_erosion_d128x1x128_output[index(p) as usize] as f32);
        spline_index_13 = binary_search(spline_coordinates_98, (coordinate_13 as f32));
        spline_values_14 = [6.3_f32, 4.69_f32];
        coordinate_14 = (buffers.minecraft_overworld_ridges_d128x1x128_output[index(p) as usize] as f32);
        spline_index_14 = binary_search(spline_coordinates_99, (coordinate_14 as f32));
        spline_values_15 = [advanced_hermite(spline_coordinates_99, spline_values_14, spline_derivatives_99, (coordinate_14 as f32), (spline_index_14 as i32)), 1.56_f32];
        coordinate_15 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_15 = binary_search(spline_coordinates_100, (coordinate_15 as f32));
        spline_values_16 = [advanced_hermite(spline_coordinates_99, spline_values_14, spline_derivatives_99, (coordinate_14 as f32), (spline_index_14 as i32)), 1.37_f32];
        coordinate_16 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_16 = binary_search(spline_coordinates_101, (coordinate_16 as f32));
        spline_values_17 = [advanced_hermite(spline_coordinates_99, spline_values_14, spline_derivatives_99, (coordinate_14 as f32), (spline_index_14 as i32)), advanced_hermite(spline_coordinates_86, spline_values_1, spline_derivatives_86, (coordinate_1 as f32), (spline_index_1 as i32)), advanced_hermite(spline_coordinates_99, spline_values_14, spline_derivatives_99, (coordinate_14 as f32), (spline_index_14 as i32)), advanced_hermite(spline_coordinates_99, spline_values_14, spline_derivatives_99, (coordinate_14 as f32), (spline_index_14 as i32)), advanced_hermite(spline_coordinates_87, spline_values_2, spline_derivatives_87, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_99, spline_values_14, spline_derivatives_99, (coordinate_14 as f32), (spline_index_14 as i32)), advanced_hermite(spline_coordinates_100, spline_values_15, spline_derivatives_100, (coordinate_15 as f32), (spline_index_15 as i32)), advanced_hermite(spline_coordinates_100, spline_values_15, spline_derivatives_100, (coordinate_15 as f32), (spline_index_15 as i32)), advanced_hermite(spline_coordinates_101, spline_values_16, spline_derivatives_101, (coordinate_16 as f32), (spline_index_16 as i32)), advanced_hermite(spline_coordinates_101, spline_values_16, spline_derivatives_101, (coordinate_16 as f32), (spline_index_16 as i32)), 4.69_f32];
        coordinate_17 = (buffers.minecraft_overworld_erosion_d128x1x128_output[index(p) as usize] as f32);
        spline_index_17 = binary_search(spline_coordinates_102, (coordinate_17 as f32));
        spline_values_18 = [3.95_f32, advanced_hermite(spline_coordinates_90, spline_values_5, spline_derivatives_90, (coordinate_5 as f32), (spline_index_5 as i32)), advanced_hermite(spline_coordinates_94, spline_values_9, spline_derivatives_94, (coordinate_9 as f32), (spline_index_9 as i32)), advanced_hermite(spline_coordinates_98, spline_values_13, spline_derivatives_98, (coordinate_13 as f32), (spline_index_13 as i32)), advanced_hermite(spline_coordinates_102, spline_values_17, spline_derivatives_102, (coordinate_17 as f32), (spline_index_17 as i32))];
        coordinate_18 = (buffers.minecraft_overworld_continents_d128x1x128_output[index(p) as usize] as f32);
        spline_index_18 = binary_search(spline_coordinates_103, (coordinate_18 as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_103, spline_values_18, spline_derivatives_103, (coordinate_18 as f32), (spline_index_18 as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        return (10_f64 + (1_f64 * (-10_f64 + spline_result_f64)));
    }
    
    #[inline]
    fn density_function_Cache2d_54_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 2];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_values_1: [f32; 2];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 6];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 5];
        let mut coordinate_3: f32;
        let mut spline_index_3: i32;
        let mut spline_values_4: [f32; 5];
        let mut coordinate_4: f32;
        let mut spline_index_4: i32;
        let mut spline_values_5: [f32; 5];
        let mut coordinate_5: f32;
        let mut spline_index_5: i32;
        let mut spline_values_6: [f32; 5];
        let mut coordinate_6: f32;
        let mut spline_index_6: i32;
        let mut spline_values_7: [f32; 7];
        let mut coordinate_7: f32;
        let mut spline_index_7: i32;
        let mut spline_values_8: [f32; 5];
        let mut coordinate_8: f32;
        let mut spline_index_8: i32;
        let mut spline_values_9: [f32; 5];
        let mut coordinate_9: f32;
        let mut spline_index_9: i32;
        let mut spline_values_10: [f32; 5];
        let mut coordinate_10: f32;
        let mut spline_index_10: i32;
        let mut spline_values_11: [f32; 5];
        let mut coordinate_11: f32;
        let mut spline_index_11: i32;
        let mut spline_values_12: [f32; 7];
        let mut coordinate_12: f32;
        let mut spline_index_12: i32;
        let mut spline_values_13: [f32; 3];
        let mut coordinate_13: f32;
        let mut spline_index_13: i32;
        let mut spline_values_14: [f32; 3];
        let mut coordinate_14: f32;
        let mut spline_index_14: i32;
        let mut spline_values_15: [f32; 3];
        let mut coordinate_15: f32;
        let mut spline_index_15: i32;
        let mut spline_values_16: [f32; 5];
        let mut coordinate_16: f32;
        let mut spline_index_16: i32;
        let mut spline_values_17: [f32; 5];
        let mut coordinate_17: f32;
        let mut spline_index_17: i32;
        let mut spline_values_18: [f32; 3];
        let mut coordinate_18: f32;
        let mut spline_index_18: i32;
        let mut spline_values_19: [f32; 11];
        let mut coordinate_19: f32;
        let mut spline_index_19: i32;
        let mut spline_values_20: [f32; 3];
        let mut coordinate_20: f32;
        let mut spline_index_20: i32;
        let mut spline_values_21: [f32; 3];
        let mut coordinate_21: f32;
        let mut spline_index_21: i32;
        let mut spline_values_22: [f32; 5];
        let mut coordinate_22: f32;
        let mut spline_index_22: i32;
        let mut spline_values_23: [f32; 5];
        let mut coordinate_23: f32;
        let mut spline_index_23: i32;
        let mut spline_values_24: [f32; 5];
        let mut coordinate_24: f32;
        let mut spline_index_24: i32;
        let mut spline_values_25: [f32; 3];
        let mut coordinate_25: f32;
        let mut spline_index_25: i32;
        let mut spline_values_26: [f32; 5];
        let mut coordinate_26: f32;
        let mut spline_index_26: i32;
        let mut spline_values_27: [f32; 11];
        let mut coordinate_27: f32;
        let mut spline_index_27: i32;
        let mut spline_values_28: [f32; 10];
        let mut coordinate_28: f32;
        let mut spline_index_28: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [-0.08880186_f32, 0.69000006_f32];
        coordinate = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index = binary_search(spline_coordinates_56, (coordinate as f32));
        spline_values_1 = [-0.115760356_f32, 0.6400001_f32];
        coordinate_1 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_1 = binary_search(spline_coordinates_57, (coordinate_1 as f32));
        spline_values_2 = [-0.2222_f32, -0.2222_f32, 0_f32, 0.000000029802322_f32, 0.000000029802322_f32, 0.100000024_f32];
        coordinate_2 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_2 = binary_search(spline_coordinates_58, (coordinate_2 as f32));
        spline_values_3 = [-0.3_f32, 0.05_f32, 0.05_f32, 0.05_f32, 0.060000002_f32];
        coordinate_3 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_3 = binary_search(spline_coordinates_59, (coordinate_3 as f32));
        spline_values_4 = [-0.15_f32, 0_f32, 0_f32, 0.05_f32, 0.060000002_f32];
        coordinate_4 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_4 = binary_search(spline_coordinates_60, (coordinate_4 as f32));
        spline_values_5 = [-0.15_f32, 0_f32, 0_f32, 0_f32, 0_f32];
        coordinate_5 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_5 = binary_search(spline_coordinates_61, (coordinate_5 as f32));
        spline_values_6 = [-0.02_f32, -0.03_f32, -0.03_f32, 0_f32, 0_f32];
        coordinate_6 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_6 = binary_search(spline_coordinates_62, (coordinate_6 as f32));
        spline_values_7 = [advanced_hermite(spline_coordinates_56, spline_values, spline_derivatives_56, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_57, spline_values_1, spline_derivatives_57, (coordinate_1 as f32), (spline_index_1 as i32)), advanced_hermite(spline_coordinates_58, spline_values_2, spline_derivatives_58, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_59, spline_values_3, spline_derivatives_59, (coordinate_3 as f32), (spline_index_3 as i32)), advanced_hermite(spline_coordinates_60, spline_values_4, spline_derivatives_60, (coordinate_4 as f32), (spline_index_4 as i32)), advanced_hermite(spline_coordinates_61, spline_values_5, spline_derivatives_61, (coordinate_5 as f32), (spline_index_5 as i32)), advanced_hermite(spline_coordinates_62, spline_values_6, spline_derivatives_62, (coordinate_6 as f32), (spline_index_6 as i32))];
        coordinate_7 = (buffers.minecraft_overworld_erosion_d128x1x128_output[index(p) as usize] as f32);
        spline_index_7 = binary_search(spline_coordinates_63, (coordinate_7 as f32));
        spline_values_8 = [-0.25_f32, 0.05_f32, 0.05_f32, 0.05_f32, 0.060000002_f32];
        coordinate_8 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_8 = binary_search(spline_coordinates_64, (coordinate_8 as f32));
        spline_values_9 = [-0.1_f32, 0.001_f32, 0.003_f32, 0.05_f32, 0.060000002_f32];
        coordinate_9 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_9 = binary_search(spline_coordinates_65, (coordinate_9 as f32));
        spline_values_10 = [-0.1_f32, 0.01_f32, 0.01_f32, 0.03_f32, 0.1_f32];
        coordinate_10 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_10 = binary_search(spline_coordinates_66, (coordinate_10 as f32));
        spline_values_11 = [-0.02_f32, -0.03_f32, -0.03_f32, 0.03_f32, 0.1_f32];
        coordinate_11 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_11 = binary_search(spline_coordinates_67, (coordinate_11 as f32));
        spline_values_12 = [advanced_hermite(spline_coordinates_56, spline_values, spline_derivatives_56, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_57, spline_values_1, spline_derivatives_57, (coordinate_1 as f32), (spline_index_1 as i32)), advanced_hermite(spline_coordinates_58, spline_values_2, spline_derivatives_58, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_64, spline_values_8, spline_derivatives_64, (coordinate_8 as f32), (spline_index_8 as i32)), advanced_hermite(spline_coordinates_65, spline_values_9, spline_derivatives_65, (coordinate_9 as f32), (spline_index_9 as i32)), advanced_hermite(spline_coordinates_66, spline_values_10, spline_derivatives_66, (coordinate_10 as f32), (spline_index_10 as i32)), advanced_hermite(spline_coordinates_67, spline_values_11, spline_derivatives_67, (coordinate_11 as f32), (spline_index_11 as i32))];
        coordinate_12 = (buffers.minecraft_overworld_erosion_d128x1x128_output[index(p) as usize] as f32);
        spline_index_12 = binary_search(spline_coordinates_68, (coordinate_12 as f32));
        spline_values_13 = [0.20235021_f32, 0.7161751_f32, 1.23_f32];
        coordinate_13 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_13 = binary_search(spline_coordinates_69, (coordinate_13 as f32));
        spline_values_14 = [0.2_f32, 0.44682026_f32, 0.88_f32];
        coordinate_14 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_14 = binary_search(spline_coordinates_70, (coordinate_14 as f32));
        spline_values_15 = [0.2_f32, 0.30829495_f32, 0.70000005_f32];
        coordinate_15 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_15 = binary_search(spline_coordinates_71, (coordinate_15 as f32));
        spline_values_16 = [-0.25_f32, 0.35_f32, 0.35_f32, 0.35_f32, 0.42000002_f32];
        coordinate_16 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_16 = binary_search(spline_coordinates_72, (coordinate_16 as f32));
        spline_values_17 = [-0.1_f32, 0.0069999998_f32, 0.021_f32, 0.35_f32, 0.42000002_f32];
        coordinate_17 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_17 = binary_search(spline_coordinates_73, (coordinate_17 as f32));
        spline_values_18 = [-0.1_f32, advanced_hermite(spline_coordinates_66, spline_values_10, spline_derivatives_66, (coordinate_10 as f32), (spline_index_10 as i32)), 0.17_f32];
        coordinate_18 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_18 = binary_search(spline_coordinates_74, (coordinate_18 as f32));
        spline_values_19 = [advanced_hermite(spline_coordinates_69, spline_values_13, spline_derivatives_69, (coordinate_13 as f32), (spline_index_13 as i32)), advanced_hermite(spline_coordinates_70, spline_values_14, spline_derivatives_70, (coordinate_14 as f32), (spline_index_14 as i32)), advanced_hermite(spline_coordinates_71, spline_values_15, spline_derivatives_71, (coordinate_15 as f32), (spline_index_15 as i32)), advanced_hermite(spline_coordinates_72, spline_values_16, spline_derivatives_72, (coordinate_16 as f32), (spline_index_16 as i32)), advanced_hermite(spline_coordinates_73, spline_values_17, spline_derivatives_73, (coordinate_17 as f32), (spline_index_17 as i32)), advanced_hermite(spline_coordinates_66, spline_values_10, spline_derivatives_66, (coordinate_10 as f32), (spline_index_10 as i32)), advanced_hermite(spline_coordinates_66, spline_values_10, spline_derivatives_66, (coordinate_10 as f32), (spline_index_10 as i32)), advanced_hermite(spline_coordinates_74, spline_values_18, spline_derivatives_74, (coordinate_18 as f32), (spline_index_18 as i32)), advanced_hermite(spline_coordinates_74, spline_values_18, spline_derivatives_74, (coordinate_18 as f32), (spline_index_18 as i32)), advanced_hermite(spline_coordinates_66, spline_values_10, spline_derivatives_66, (coordinate_10 as f32), (spline_index_10 as i32)), advanced_hermite(spline_coordinates_67, spline_values_11, spline_derivatives_67, (coordinate_11 as f32), (spline_index_11 as i32))];
        coordinate_19 = (buffers.minecraft_overworld_erosion_d128x1x128_output[index(p) as usize] as f32);
        spline_index_19 = binary_search(spline_coordinates_75, (coordinate_19 as f32));
        spline_values_20 = [0.34792626_f32, 0.9239631_f32, 1.5_f32];
        coordinate_20 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_20 = binary_search(spline_coordinates_76, (coordinate_20 as f32));
        spline_values_21 = [0.2_f32, 0.5391705_f32, 1_f32];
        coordinate_21 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_21 = binary_search(spline_coordinates_77, (coordinate_21 as f32));
        spline_values_22 = [-0.2_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.6_f32];
        coordinate_22 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_22 = binary_search(spline_coordinates_78, (coordinate_22 as f32));
        spline_values_23 = [-0.05_f32, 0.01_f32, 0.03_f32, 0.5_f32, 0.6_f32];
        coordinate_23 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_23 = binary_search(spline_coordinates_79, (coordinate_23 as f32));
        spline_values_24 = [-0.05_f32, 0.01_f32, 0.01_f32, 0.03_f32, 0.1_f32];
        coordinate_24 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_24 = binary_search(spline_coordinates_80, (coordinate_24 as f32));
        spline_values_25 = [-0.05_f32, advanced_hermite(spline_coordinates_80, spline_values_24, spline_derivatives_80, (coordinate_24 as f32), (spline_index_24 as i32)), 0.17_f32];
        coordinate_25 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_25 = binary_search(spline_coordinates_81, (coordinate_25 as f32));
        spline_values_26 = [-0.02_f32, 0.01_f32, 0.01_f32, 0.03_f32, 0.1_f32];
        coordinate_26 = (buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] as f32);
        spline_index_26 = binary_search(spline_coordinates_82, (coordinate_26 as f32));
        spline_values_27 = [advanced_hermite(spline_coordinates_76, spline_values_20, spline_derivatives_76, (coordinate_20 as f32), (spline_index_20 as i32)), advanced_hermite(spline_coordinates_77, spline_values_21, spline_derivatives_77, (coordinate_21 as f32), (spline_index_21 as i32)), advanced_hermite(spline_coordinates_77, spline_values_21, spline_derivatives_77, (coordinate_21 as f32), (spline_index_21 as i32)), advanced_hermite(spline_coordinates_78, spline_values_22, spline_derivatives_78, (coordinate_22 as f32), (spline_index_22 as i32)), advanced_hermite(spline_coordinates_79, spline_values_23, spline_derivatives_79, (coordinate_23 as f32), (spline_index_23 as i32)), advanced_hermite(spline_coordinates_80, spline_values_24, spline_derivatives_80, (coordinate_24 as f32), (spline_index_24 as i32)), advanced_hermite(spline_coordinates_80, spline_values_24, spline_derivatives_80, (coordinate_24 as f32), (spline_index_24 as i32)), advanced_hermite(spline_coordinates_81, spline_values_25, spline_derivatives_81, (coordinate_25 as f32), (spline_index_25 as i32)), advanced_hermite(spline_coordinates_81, spline_values_25, spline_derivatives_81, (coordinate_25 as f32), (spline_index_25 as i32)), advanced_hermite(spline_coordinates_80, spline_values_24, spline_derivatives_80, (coordinate_24 as f32), (spline_index_24 as i32)), advanced_hermite(spline_coordinates_82, spline_values_26, spline_derivatives_82, (coordinate_26 as f32), (spline_index_26 as i32))];
        coordinate_27 = (buffers.minecraft_overworld_erosion_d128x1x128_output[index(p) as usize] as f32);
        spline_index_27 = binary_search(spline_coordinates_83, (coordinate_27 as f32));
        spline_values_28 = [0.044_f32, -0.2222_f32, -0.2222_f32, -0.12_f32, -0.12_f32, advanced_hermite(spline_coordinates_63, spline_values_7, spline_derivatives_63, (coordinate_7 as f32), (spline_index_7 as i32)), advanced_hermite(spline_coordinates_63, spline_values_7, spline_derivatives_63, (coordinate_7 as f32), (spline_index_7 as i32)), advanced_hermite(spline_coordinates_68, spline_values_12, spline_derivatives_68, (coordinate_12 as f32), (spline_index_12 as i32)), advanced_hermite(spline_coordinates_75, spline_values_19, spline_derivatives_75, (coordinate_19 as f32), (spline_index_19 as i32)), advanced_hermite(spline_coordinates_83, spline_values_27, spline_derivatives_83, (coordinate_27 as f32), (spline_index_27 as i32))];
        coordinate_28 = (buffers.minecraft_overworld_continents_d128x1x128_output[index(p) as usize] as f32);
        spline_index_28 = binary_search(spline_coordinates_84, (coordinate_28 as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_84, spline_values_28, spline_derivatives_84, (coordinate_28 as f32), (spline_index_28 as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        return ((0_f64 * (1_f64 + (-1_f64 * 1_f64))) + ((-0.5037500262260437_f64 + spline_result_f64) * 1_f64));
    }
    
    #[inline]
    fn density_function_Cache2d_1_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 2];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_values_1: [f32; 2];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 6];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 5];
        let mut coordinate_3: f32;
        let mut spline_index_3: i32;
        let mut spline_values_4: [f32; 5];
        let mut coordinate_4: f32;
        let mut spline_index_4: i32;
        let mut spline_values_5: [f32; 5];
        let mut coordinate_5: f32;
        let mut spline_index_5: i32;
        let mut spline_values_6: [f32; 5];
        let mut coordinate_6: f32;
        let mut spline_index_6: i32;
        let mut spline_values_7: [f32; 7];
        let mut coordinate_7: f32;
        let mut spline_index_7: i32;
        let mut spline_values_8: [f32; 5];
        let mut coordinate_8: f32;
        let mut spline_index_8: i32;
        let mut spline_values_9: [f32; 5];
        let mut coordinate_9: f32;
        let mut spline_index_9: i32;
        let mut spline_values_10: [f32; 5];
        let mut coordinate_10: f32;
        let mut spline_index_10: i32;
        let mut spline_values_11: [f32; 5];
        let mut coordinate_11: f32;
        let mut spline_index_11: i32;
        let mut spline_values_12: [f32; 7];
        let mut coordinate_12: f32;
        let mut spline_index_12: i32;
        let mut spline_values_13: [f32; 3];
        let mut coordinate_13: f32;
        let mut spline_index_13: i32;
        let mut spline_values_14: [f32; 3];
        let mut coordinate_14: f32;
        let mut spline_index_14: i32;
        let mut spline_values_15: [f32; 3];
        let mut coordinate_15: f32;
        let mut spline_index_15: i32;
        let mut spline_values_16: [f32; 5];
        let mut coordinate_16: f32;
        let mut spline_index_16: i32;
        let mut spline_values_17: [f32; 5];
        let mut coordinate_17: f32;
        let mut spline_index_17: i32;
        let mut spline_values_18: [f32; 3];
        let mut coordinate_18: f32;
        let mut spline_index_18: i32;
        let mut spline_values_19: [f32; 11];
        let mut coordinate_19: f32;
        let mut spline_index_19: i32;
        let mut spline_values_20: [f32; 3];
        let mut coordinate_20: f32;
        let mut spline_index_20: i32;
        let mut spline_values_21: [f32; 3];
        let mut coordinate_21: f32;
        let mut spline_index_21: i32;
        let mut spline_values_22: [f32; 5];
        let mut coordinate_22: f32;
        let mut spline_index_22: i32;
        let mut spline_values_23: [f32; 5];
        let mut coordinate_23: f32;
        let mut spline_index_23: i32;
        let mut spline_values_24: [f32; 5];
        let mut coordinate_24: f32;
        let mut spline_index_24: i32;
        let mut spline_values_25: [f32; 3];
        let mut coordinate_25: f32;
        let mut spline_index_25: i32;
        let mut spline_values_26: [f32; 5];
        let mut coordinate_26: f32;
        let mut spline_index_26: i32;
        let mut spline_values_27: [f32; 11];
        let mut coordinate_27: f32;
        let mut spline_index_27: i32;
        let mut spline_values_28: [f32; 10];
        let mut coordinate_28: f32;
        let mut spline_index_28: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [-0.08880186_f32, 0.69000006_f32];
        coordinate = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index = binary_search(spline_coordinates, (coordinate as f32));
        spline_values_1 = [-0.115760356_f32, 0.6400001_f32];
        coordinate_1 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_1 = binary_search(spline_coordinates_1, (coordinate_1 as f32));
        spline_values_2 = [-0.2222_f32, -0.2222_f32, 0_f32, 0.000000029802322_f32, 0.000000029802322_f32, 0.100000024_f32];
        coordinate_2 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_2 = binary_search(spline_coordinates_2, (coordinate_2 as f32));
        spline_values_3 = [-0.3_f32, 0.05_f32, 0.05_f32, 0.05_f32, 0.060000002_f32];
        coordinate_3 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_3 = binary_search(spline_coordinates_3, (coordinate_3 as f32));
        spline_values_4 = [-0.15_f32, 0_f32, 0_f32, 0.05_f32, 0.060000002_f32];
        coordinate_4 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_4 = binary_search(spline_coordinates_4, (coordinate_4 as f32));
        spline_values_5 = [-0.15_f32, 0_f32, 0_f32, 0_f32, 0_f32];
        coordinate_5 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_5 = binary_search(spline_coordinates_5, (coordinate_5 as f32));
        spline_values_6 = [-0.02_f32, -0.03_f32, -0.03_f32, 0_f32, 0_f32];
        coordinate_6 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_6 = binary_search(spline_coordinates_6, (coordinate_6 as f32));
        spline_values_7 = [advanced_hermite(spline_coordinates, spline_values, spline_derivatives, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_1, spline_values_1, spline_derivatives_1, (coordinate_1 as f32), (spline_index_1 as i32)), advanced_hermite(spline_coordinates_2, spline_values_2, spline_derivatives_2, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_3, spline_values_3, spline_derivatives_3, (coordinate_3 as f32), (spline_index_3 as i32)), advanced_hermite(spline_coordinates_4, spline_values_4, spline_derivatives_4, (coordinate_4 as f32), (spline_index_4 as i32)), advanced_hermite(spline_coordinates_5, spline_values_5, spline_derivatives_5, (coordinate_5 as f32), (spline_index_5 as i32)), advanced_hermite(spline_coordinates_6, spline_values_6, spline_derivatives_6, (coordinate_6 as f32), (spline_index_6 as i32))];
        coordinate_7 = (buffers.minecraft_overworld_erosion_d5x1x5_output[index(p) as usize] as f32);
        spline_index_7 = binary_search(spline_coordinates_7, (coordinate_7 as f32));
        spline_values_8 = [-0.25_f32, 0.05_f32, 0.05_f32, 0.05_f32, 0.060000002_f32];
        coordinate_8 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_8 = binary_search(spline_coordinates_8, (coordinate_8 as f32));
        spline_values_9 = [-0.1_f32, 0.001_f32, 0.003_f32, 0.05_f32, 0.060000002_f32];
        coordinate_9 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_9 = binary_search(spline_coordinates_9, (coordinate_9 as f32));
        spline_values_10 = [-0.1_f32, 0.01_f32, 0.01_f32, 0.03_f32, 0.1_f32];
        coordinate_10 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_10 = binary_search(spline_coordinates_10, (coordinate_10 as f32));
        spline_values_11 = [-0.02_f32, -0.03_f32, -0.03_f32, 0.03_f32, 0.1_f32];
        coordinate_11 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_11 = binary_search(spline_coordinates_11, (coordinate_11 as f32));
        spline_values_12 = [advanced_hermite(spline_coordinates, spline_values, spline_derivatives, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_1, spline_values_1, spline_derivatives_1, (coordinate_1 as f32), (spline_index_1 as i32)), advanced_hermite(spline_coordinates_2, spline_values_2, spline_derivatives_2, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_8, spline_values_8, spline_derivatives_8, (coordinate_8 as f32), (spline_index_8 as i32)), advanced_hermite(spline_coordinates_9, spline_values_9, spline_derivatives_9, (coordinate_9 as f32), (spline_index_9 as i32)), advanced_hermite(spline_coordinates_10, spline_values_10, spline_derivatives_10, (coordinate_10 as f32), (spline_index_10 as i32)), advanced_hermite(spline_coordinates_11, spline_values_11, spline_derivatives_11, (coordinate_11 as f32), (spline_index_11 as i32))];
        coordinate_12 = (buffers.minecraft_overworld_erosion_d5x1x5_output[index(p) as usize] as f32);
        spline_index_12 = binary_search(spline_coordinates_12, (coordinate_12 as f32));
        spline_values_13 = [0.20235021_f32, 0.7161751_f32, 1.23_f32];
        coordinate_13 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_13 = binary_search(spline_coordinates_13, (coordinate_13 as f32));
        spline_values_14 = [0.2_f32, 0.44682026_f32, 0.88_f32];
        coordinate_14 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_14 = binary_search(spline_coordinates_14, (coordinate_14 as f32));
        spline_values_15 = [0.2_f32, 0.30829495_f32, 0.70000005_f32];
        coordinate_15 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_15 = binary_search(spline_coordinates_15, (coordinate_15 as f32));
        spline_values_16 = [-0.25_f32, 0.35_f32, 0.35_f32, 0.35_f32, 0.42000002_f32];
        coordinate_16 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_16 = binary_search(spline_coordinates_16, (coordinate_16 as f32));
        spline_values_17 = [-0.1_f32, 0.0069999998_f32, 0.021_f32, 0.35_f32, 0.42000002_f32];
        coordinate_17 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_17 = binary_search(spline_coordinates_17, (coordinate_17 as f32));
        spline_values_18 = [-0.1_f32, advanced_hermite(spline_coordinates_10, spline_values_10, spline_derivatives_10, (coordinate_10 as f32), (spline_index_10 as i32)), 0.17_f32];
        coordinate_18 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_18 = binary_search(spline_coordinates_18, (coordinate_18 as f32));
        spline_values_19 = [advanced_hermite(spline_coordinates_13, spline_values_13, spline_derivatives_13, (coordinate_13 as f32), (spline_index_13 as i32)), advanced_hermite(spline_coordinates_14, spline_values_14, spline_derivatives_14, (coordinate_14 as f32), (spline_index_14 as i32)), advanced_hermite(spline_coordinates_15, spline_values_15, spline_derivatives_15, (coordinate_15 as f32), (spline_index_15 as i32)), advanced_hermite(spline_coordinates_16, spline_values_16, spline_derivatives_16, (coordinate_16 as f32), (spline_index_16 as i32)), advanced_hermite(spline_coordinates_17, spline_values_17, spline_derivatives_17, (coordinate_17 as f32), (spline_index_17 as i32)), advanced_hermite(spline_coordinates_10, spline_values_10, spline_derivatives_10, (coordinate_10 as f32), (spline_index_10 as i32)), advanced_hermite(spline_coordinates_10, spline_values_10, spline_derivatives_10, (coordinate_10 as f32), (spline_index_10 as i32)), advanced_hermite(spline_coordinates_18, spline_values_18, spline_derivatives_18, (coordinate_18 as f32), (spline_index_18 as i32)), advanced_hermite(spline_coordinates_18, spline_values_18, spline_derivatives_18, (coordinate_18 as f32), (spline_index_18 as i32)), advanced_hermite(spline_coordinates_10, spline_values_10, spline_derivatives_10, (coordinate_10 as f32), (spline_index_10 as i32)), advanced_hermite(spline_coordinates_11, spline_values_11, spline_derivatives_11, (coordinate_11 as f32), (spline_index_11 as i32))];
        coordinate_19 = (buffers.minecraft_overworld_erosion_d5x1x5_output[index(p) as usize] as f32);
        spline_index_19 = binary_search(spline_coordinates_19, (coordinate_19 as f32));
        spline_values_20 = [0.34792626_f32, 0.9239631_f32, 1.5_f32];
        coordinate_20 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_20 = binary_search(spline_coordinates_20, (coordinate_20 as f32));
        spline_values_21 = [0.2_f32, 0.5391705_f32, 1_f32];
        coordinate_21 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_21 = binary_search(spline_coordinates_21, (coordinate_21 as f32));
        spline_values_22 = [-0.2_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.6_f32];
        coordinate_22 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_22 = binary_search(spline_coordinates_22, (coordinate_22 as f32));
        spline_values_23 = [-0.05_f32, 0.01_f32, 0.03_f32, 0.5_f32, 0.6_f32];
        coordinate_23 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_23 = binary_search(spline_coordinates_23, (coordinate_23 as f32));
        spline_values_24 = [-0.05_f32, 0.01_f32, 0.01_f32, 0.03_f32, 0.1_f32];
        coordinate_24 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_24 = binary_search(spline_coordinates_24, (coordinate_24 as f32));
        spline_values_25 = [-0.05_f32, advanced_hermite(spline_coordinates_24, spline_values_24, spline_derivatives_24, (coordinate_24 as f32), (spline_index_24 as i32)), 0.17_f32];
        coordinate_25 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_25 = binary_search(spline_coordinates_25, (coordinate_25 as f32));
        spline_values_26 = [-0.02_f32, 0.01_f32, 0.01_f32, 0.03_f32, 0.1_f32];
        coordinate_26 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_26 = binary_search(spline_coordinates_26, (coordinate_26 as f32));
        spline_values_27 = [advanced_hermite(spline_coordinates_20, spline_values_20, spline_derivatives_20, (coordinate_20 as f32), (spline_index_20 as i32)), advanced_hermite(spline_coordinates_21, spline_values_21, spline_derivatives_21, (coordinate_21 as f32), (spline_index_21 as i32)), advanced_hermite(spline_coordinates_21, spline_values_21, spline_derivatives_21, (coordinate_21 as f32), (spline_index_21 as i32)), advanced_hermite(spline_coordinates_22, spline_values_22, spline_derivatives_22, (coordinate_22 as f32), (spline_index_22 as i32)), advanced_hermite(spline_coordinates_23, spline_values_23, spline_derivatives_23, (coordinate_23 as f32), (spline_index_23 as i32)), advanced_hermite(spline_coordinates_24, spline_values_24, spline_derivatives_24, (coordinate_24 as f32), (spline_index_24 as i32)), advanced_hermite(spline_coordinates_24, spline_values_24, spline_derivatives_24, (coordinate_24 as f32), (spline_index_24 as i32)), advanced_hermite(spline_coordinates_25, spline_values_25, spline_derivatives_25, (coordinate_25 as f32), (spline_index_25 as i32)), advanced_hermite(spline_coordinates_25, spline_values_25, spline_derivatives_25, (coordinate_25 as f32), (spline_index_25 as i32)), advanced_hermite(spline_coordinates_24, spline_values_24, spline_derivatives_24, (coordinate_24 as f32), (spline_index_24 as i32)), advanced_hermite(spline_coordinates_26, spline_values_26, spline_derivatives_26, (coordinate_26 as f32), (spline_index_26 as i32))];
        coordinate_27 = (buffers.minecraft_overworld_erosion_d5x1x5_output[index(p) as usize] as f32);
        spline_index_27 = binary_search(spline_coordinates_27, (coordinate_27 as f32));
        spline_values_28 = [0.044_f32, -0.2222_f32, -0.2222_f32, -0.12_f32, -0.12_f32, advanced_hermite(spline_coordinates_7, spline_values_7, spline_derivatives_7, (coordinate_7 as f32), (spline_index_7 as i32)), advanced_hermite(spline_coordinates_7, spline_values_7, spline_derivatives_7, (coordinate_7 as f32), (spline_index_7 as i32)), advanced_hermite(spline_coordinates_12, spline_values_12, spline_derivatives_12, (coordinate_12 as f32), (spline_index_12 as i32)), advanced_hermite(spline_coordinates_19, spline_values_19, spline_derivatives_19, (coordinate_19 as f32), (spline_index_19 as i32)), advanced_hermite(spline_coordinates_27, spline_values_27, spline_derivatives_27, (coordinate_27 as f32), (spline_index_27 as i32))];
        coordinate_28 = (buffers.minecraft_overworld_continents_d5x1x5_output[index(p) as usize] as f32);
        spline_index_28 = binary_search(spline_coordinates_28, (coordinate_28 as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_28, spline_values_28, spline_derivatives_28, (coordinate_28 as f32), (spline_index_28 as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        return ((0_f64 * (1_f64 + (-1_f64 * 1_f64))) + ((-0.5037500262260437_f64 + spline_result_f64) * 1_f64));
    }
    
    #[inline]
    fn density_function_Cache2d_12_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 2];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_values_1: [f32; 3];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 2];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 3];
        let mut coordinate_3: f32;
        let mut spline_index_3: i32;
        let mut spline_values_4: [f32; 4];
        let mut coordinate_4: f32;
        let mut spline_index_4: i32;
        let mut spline_values_5: [f32; 3];
        let mut coordinate_5: f32;
        let mut spline_index_5: i32;
        let mut spline_values_6: [f32; 4];
        let mut coordinate_6: f32;
        let mut spline_index_6: i32;
        let mut spline_values_7: [f32; 3];
        let mut coordinate_7: f32;
        let mut spline_index_7: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [0.63_f32, 0.3_f32];
        coordinate = (buffers.minecraft_overworld_ridges_d5x1x5_output[index(p) as usize] as f32);
        spline_index = binary_search(spline_coordinates_29, (coordinate as f32));
        spline_values_1 = [0_f32, 0_f32, advanced_hermite(spline_coordinates_29, spline_values, spline_derivatives_29, (coordinate as f32), (spline_index as i32))];
        coordinate_1 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_1 = binary_search(spline_coordinates_30, (coordinate_1 as f32));
        spline_values_2 = [0.315_f32, 0.15_f32];
        coordinate_2 = (buffers.minecraft_overworld_ridges_d5x1x5_output[index(p) as usize] as f32);
        spline_index_2 = binary_search(spline_coordinates_31, (coordinate_2 as f32));
        spline_values_3 = [0_f32, 0_f32, advanced_hermite(spline_coordinates_31, spline_values_2, spline_derivatives_31, (coordinate_2 as f32), (spline_index_2 as i32))];
        coordinate_3 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_3 = binary_search(spline_coordinates_32, (coordinate_3 as f32));
        spline_values_4 = [advanced_hermite(spline_coordinates_30, spline_values_1, spline_derivatives_30, (coordinate_1 as f32), (spline_index_1 as i32)), advanced_hermite(spline_coordinates_32, spline_values_3, spline_derivatives_32, (coordinate_3 as f32), (spline_index_3 as i32)), advanced_hermite(spline_coordinates_32, spline_values_3, spline_derivatives_32, (coordinate_3 as f32), (spline_index_3 as i32)), 0_f32];
        coordinate_4 = (buffers.minecraft_overworld_erosion_d5x1x5_output[index(p) as usize] as f32);
        spline_index_4 = binary_search(spline_coordinates_33, (coordinate_4 as f32));
        spline_values_5 = [0_f32, advanced_hermite(spline_coordinates_29, spline_values, spline_derivatives_29, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_29, spline_values, spline_derivatives_29, (coordinate as f32), (spline_index as i32))];
        coordinate_5 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_5 = binary_search(spline_coordinates_34, (coordinate_5 as f32));
        spline_values_6 = [advanced_hermite(spline_coordinates_34, spline_values_5, spline_derivatives_34, (coordinate_5 as f32), (spline_index_5 as i32)), advanced_hermite(spline_coordinates_30, spline_values_1, spline_derivatives_30, (coordinate_1 as f32), (spline_index_1 as i32)), advanced_hermite(spline_coordinates_30, spline_values_1, spline_derivatives_30, (coordinate_1 as f32), (spline_index_1 as i32)), 0_f32];
        coordinate_6 = (buffers.minecraft_overworld_erosion_d5x1x5_output[index(p) as usize] as f32);
        spline_index_6 = binary_search(spline_coordinates_35, (coordinate_6 as f32));
        spline_values_7 = [0_f32, advanced_hermite(spline_coordinates_33, spline_values_4, spline_derivatives_33, (coordinate_4 as f32), (spline_index_4 as i32)), advanced_hermite(spline_coordinates_35, spline_values_6, spline_derivatives_35, (coordinate_6 as f32), (spline_index_6 as i32))];
        coordinate_7 = (buffers.minecraft_overworld_continents_d5x1x5_output[index(p) as usize] as f32);
        spline_index_7 = binary_search(spline_coordinates_36, (coordinate_7 as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_36, spline_values_7, spline_derivatives_36, (coordinate_7 as f32), (spline_index_7 as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        return (0_f64 + (1_f64 * (-0_f64 + spline_result_f64)));
    }
    
    #[inline]
    fn density_function_Cache2d_15_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 2];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_values_1: [f32; 2];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 2];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 2];
        let mut coordinate_3: f32;
        let mut spline_index_3: i32;
        let mut spline_values_4: [f32; 2];
        let mut coordinate_4: f32;
        let mut spline_index_4: i32;
        let mut spline_values_5: [f32; 10];
        let mut coordinate_5: f32;
        let mut spline_index_5: i32;
        let mut spline_values_6: [f32; 2];
        let mut coordinate_6: f32;
        let mut spline_index_6: i32;
        let mut spline_values_7: [f32; 2];
        let mut coordinate_7: f32;
        let mut spline_index_7: i32;
        let mut spline_values_8: [f32; 2];
        let mut coordinate_8: f32;
        let mut spline_index_8: i32;
        let mut spline_values_9: [f32; 10];
        let mut coordinate_9: f32;
        let mut spline_index_9: i32;
        let mut spline_values_10: [f32; 2];
        let mut coordinate_10: f32;
        let mut spline_index_10: i32;
        let mut spline_values_11: [f32; 2];
        let mut coordinate_11: f32;
        let mut spline_index_11: i32;
        let mut spline_values_12: [f32; 2];
        let mut coordinate_12: f32;
        let mut spline_index_12: i32;
        let mut spline_values_13: [f32; 10];
        let mut coordinate_13: f32;
        let mut spline_index_13: i32;
        let mut spline_values_14: [f32; 2];
        let mut coordinate_14: f32;
        let mut spline_index_14: i32;
        let mut spline_values_15: [f32; 2];
        let mut coordinate_15: f32;
        let mut spline_index_15: i32;
        let mut spline_values_16: [f32; 2];
        let mut coordinate_16: f32;
        let mut spline_index_16: i32;
        let mut spline_values_17: [f32; 11];
        let mut coordinate_17: f32;
        let mut spline_index_17: i32;
        let mut spline_values_18: [f32; 5];
        let mut coordinate_18: f32;
        let mut spline_index_18: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [6.3_f32, 6.25_f32];
        coordinate = (buffers.minecraft_overworld_ridges_d5x1x5_output[index(p) as usize] as f32);
        spline_index = binary_search(spline_coordinates_37, (coordinate as f32));
        spline_values_1 = [6.3_f32, 2.67_f32];
        coordinate_1 = (buffers.minecraft_overworld_ridges_d5x1x5_output[index(p) as usize] as f32);
        spline_index_1 = binary_search(spline_coordinates_38, (coordinate_1 as f32));
        spline_values_2 = [2.67_f32, 6.3_f32];
        coordinate_2 = (buffers.minecraft_overworld_ridges_d5x1x5_output[index(p) as usize] as f32);
        spline_index_2 = binary_search(spline_coordinates_39, (coordinate_2 as f32));
        spline_values_3 = [6.25_f32, 0.625_f32];
        coordinate_3 = (buffers.minecraft_overworld_ridges_d5x1x5_output[index(p) as usize] as f32);
        spline_index_3 = binary_search(spline_coordinates_40, (coordinate_3 as f32));
        spline_values_4 = [6.25_f32, advanced_hermite(spline_coordinates_40, spline_values_3, spline_derivatives_40, (coordinate_3 as f32), (spline_index_3 as i32))];
        coordinate_4 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_4 = binary_search(spline_coordinates_41, (coordinate_4 as f32));
        spline_values_5 = [advanced_hermite(spline_coordinates_37, spline_values, spline_derivatives_37, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_38, spline_values_1, spline_derivatives_38, (coordinate_1 as f32), (spline_index_1 as i32)), advanced_hermite(spline_coordinates_37, spline_values, spline_derivatives_37, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_37, spline_values, spline_derivatives_37, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_39, spline_values_2, spline_derivatives_39, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_37, spline_values, spline_derivatives_37, (coordinate as f32), (spline_index as i32)), 6.25_f32, advanced_hermite(spline_coordinates_41, spline_values_4, spline_derivatives_41, (coordinate_4 as f32), (spline_index_4 as i32)), advanced_hermite(spline_coordinates_41, spline_values_4, spline_derivatives_41, (coordinate_4 as f32), (spline_index_4 as i32)), 6.25_f32];
        coordinate_5 = (buffers.minecraft_overworld_erosion_d5x1x5_output[index(p) as usize] as f32);
        spline_index_5 = binary_search(spline_coordinates_42, (coordinate_5 as f32));
        spline_values_6 = [6.3_f32, 5.47_f32];
        coordinate_6 = (buffers.minecraft_overworld_ridges_d5x1x5_output[index(p) as usize] as f32);
        spline_index_6 = binary_search(spline_coordinates_43, (coordinate_6 as f32));
        spline_values_7 = [5.47_f32, 0.625_f32];
        coordinate_7 = (buffers.minecraft_overworld_ridges_d5x1x5_output[index(p) as usize] as f32);
        spline_index_7 = binary_search(spline_coordinates_44, (coordinate_7 as f32));
        spline_values_8 = [5.47_f32, advanced_hermite(spline_coordinates_44, spline_values_7, spline_derivatives_44, (coordinate_7 as f32), (spline_index_7 as i32))];
        coordinate_8 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_8 = binary_search(spline_coordinates_45, (coordinate_8 as f32));
        spline_values_9 = [advanced_hermite(spline_coordinates_43, spline_values_6, spline_derivatives_43, (coordinate_6 as f32), (spline_index_6 as i32)), advanced_hermite(spline_coordinates_38, spline_values_1, spline_derivatives_38, (coordinate_1 as f32), (spline_index_1 as i32)), advanced_hermite(spline_coordinates_43, spline_values_6, spline_derivatives_43, (coordinate_6 as f32), (spline_index_6 as i32)), advanced_hermite(spline_coordinates_43, spline_values_6, spline_derivatives_43, (coordinate_6 as f32), (spline_index_6 as i32)), advanced_hermite(spline_coordinates_39, spline_values_2, spline_derivatives_39, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_43, spline_values_6, spline_derivatives_43, (coordinate_6 as f32), (spline_index_6 as i32)), 5.47_f32, advanced_hermite(spline_coordinates_45, spline_values_8, spline_derivatives_45, (coordinate_8 as f32), (spline_index_8 as i32)), advanced_hermite(spline_coordinates_45, spline_values_8, spline_derivatives_45, (coordinate_8 as f32), (spline_index_8 as i32)), 5.47_f32];
        coordinate_9 = (buffers.minecraft_overworld_erosion_d5x1x5_output[index(p) as usize] as f32);
        spline_index_9 = binary_search(spline_coordinates_46, (coordinate_9 as f32));
        spline_values_10 = [6.3_f32, 5.08_f32];
        coordinate_10 = (buffers.minecraft_overworld_ridges_d5x1x5_output[index(p) as usize] as f32);
        spline_index_10 = binary_search(spline_coordinates_47, (coordinate_10 as f32));
        spline_values_11 = [5.08_f32, 0.625_f32];
        coordinate_11 = (buffers.minecraft_overworld_ridges_d5x1x5_output[index(p) as usize] as f32);
        spline_index_11 = binary_search(spline_coordinates_48, (coordinate_11 as f32));
        spline_values_12 = [5.08_f32, advanced_hermite(spline_coordinates_48, spline_values_11, spline_derivatives_48, (coordinate_11 as f32), (spline_index_11 as i32))];
        coordinate_12 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_12 = binary_search(spline_coordinates_49, (coordinate_12 as f32));
        spline_values_13 = [advanced_hermite(spline_coordinates_47, spline_values_10, spline_derivatives_47, (coordinate_10 as f32), (spline_index_10 as i32)), advanced_hermite(spline_coordinates_38, spline_values_1, spline_derivatives_38, (coordinate_1 as f32), (spline_index_1 as i32)), advanced_hermite(spline_coordinates_47, spline_values_10, spline_derivatives_47, (coordinate_10 as f32), (spline_index_10 as i32)), advanced_hermite(spline_coordinates_47, spline_values_10, spline_derivatives_47, (coordinate_10 as f32), (spline_index_10 as i32)), advanced_hermite(spline_coordinates_39, spline_values_2, spline_derivatives_39, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_47, spline_values_10, spline_derivatives_47, (coordinate_10 as f32), (spline_index_10 as i32)), 5.08_f32, advanced_hermite(spline_coordinates_49, spline_values_12, spline_derivatives_49, (coordinate_12 as f32), (spline_index_12 as i32)), advanced_hermite(spline_coordinates_49, spline_values_12, spline_derivatives_49, (coordinate_12 as f32), (spline_index_12 as i32)), 5.08_f32];
        coordinate_13 = (buffers.minecraft_overworld_erosion_d5x1x5_output[index(p) as usize] as f32);
        spline_index_13 = binary_search(spline_coordinates_50, (coordinate_13 as f32));
        spline_values_14 = [6.3_f32, 4.69_f32];
        coordinate_14 = (buffers.minecraft_overworld_ridges_d5x1x5_output[index(p) as usize] as f32);
        spline_index_14 = binary_search(spline_coordinates_51, (coordinate_14 as f32));
        spline_values_15 = [advanced_hermite(spline_coordinates_51, spline_values_14, spline_derivatives_51, (coordinate_14 as f32), (spline_index_14 as i32)), 1.56_f32];
        coordinate_15 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_15 = binary_search(spline_coordinates_52, (coordinate_15 as f32));
        spline_values_16 = [advanced_hermite(spline_coordinates_51, spline_values_14, spline_derivatives_51, (coordinate_14 as f32), (spline_index_14 as i32)), 1.37_f32];
        coordinate_16 = (buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] as f32);
        spline_index_16 = binary_search(spline_coordinates_53, (coordinate_16 as f32));
        spline_values_17 = [advanced_hermite(spline_coordinates_51, spline_values_14, spline_derivatives_51, (coordinate_14 as f32), (spline_index_14 as i32)), advanced_hermite(spline_coordinates_38, spline_values_1, spline_derivatives_38, (coordinate_1 as f32), (spline_index_1 as i32)), advanced_hermite(spline_coordinates_51, spline_values_14, spline_derivatives_51, (coordinate_14 as f32), (spline_index_14 as i32)), advanced_hermite(spline_coordinates_51, spline_values_14, spline_derivatives_51, (coordinate_14 as f32), (spline_index_14 as i32)), advanced_hermite(spline_coordinates_39, spline_values_2, spline_derivatives_39, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_51, spline_values_14, spline_derivatives_51, (coordinate_14 as f32), (spline_index_14 as i32)), advanced_hermite(spline_coordinates_52, spline_values_15, spline_derivatives_52, (coordinate_15 as f32), (spline_index_15 as i32)), advanced_hermite(spline_coordinates_52, spline_values_15, spline_derivatives_52, (coordinate_15 as f32), (spline_index_15 as i32)), advanced_hermite(spline_coordinates_53, spline_values_16, spline_derivatives_53, (coordinate_16 as f32), (spline_index_16 as i32)), advanced_hermite(spline_coordinates_53, spline_values_16, spline_derivatives_53, (coordinate_16 as f32), (spline_index_16 as i32)), 4.69_f32];
        coordinate_17 = (buffers.minecraft_overworld_erosion_d5x1x5_output[index(p) as usize] as f32);
        spline_index_17 = binary_search(spline_coordinates_54, (coordinate_17 as f32));
        spline_values_18 = [3.95_f32, advanced_hermite(spline_coordinates_42, spline_values_5, spline_derivatives_42, (coordinate_5 as f32), (spline_index_5 as i32)), advanced_hermite(spline_coordinates_46, spline_values_9, spline_derivatives_46, (coordinate_9 as f32), (spline_index_9 as i32)), advanced_hermite(spline_coordinates_50, spline_values_13, spline_derivatives_50, (coordinate_13 as f32), (spline_index_13 as i32)), advanced_hermite(spline_coordinates_54, spline_values_17, spline_derivatives_54, (coordinate_17 as f32), (spline_index_17 as i32))];
        coordinate_18 = (buffers.minecraft_overworld_continents_d5x1x5_output[index(p) as usize] as f32);
        spline_index_18 = binary_search(spline_coordinates_55, (coordinate_18 as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_55, spline_values_18, spline_derivatives_55, (coordinate_18 as f32), (spline_index_18 as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        return (10_f64 + (1_f64 * (-10_f64 + spline_result_f64)));
    }
    
    #[inline]
    fn density_function_Add_0_d5x49x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut var_1: f64;
        let mut var_2: f64;
        let mut var_3: f64;
        let mut var_4: f64;
        let mut var_5: f64;
        let mut var_6: f64;
        let mut var_7: f64;
        let mut var_8: f64;
        let mut var_9: f64;
        let mut var_10: f64;
        let mut var_11: f64;
        let mut var_12: f64;
        let mut var_13: f64;
        let mut var_14: f64;
        let mut var_15: f64;
        let mut var_16: f64;
        let mut var_17: f64;
        let mut var_18: f64;
        let mut var_19: f64;
        let mut var_20: f64;
        let mut var_21: f64;
        let mut var_22: f64;
        let mut var_23: f64;
        let mut var_24: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = clamp(((rpos3.y - -64_f64) / 24_f64), 0_f64, 1_f64);
        var_1 = clamp(((rpos3.y - 240_f64) / 16_f64), 0_f64, 1_f64);
        var_2 = clamp(((rpos3.y - -64_f64) / 384_f64), 0_f64, 1_f64);
        var_3 = buffers.minecraft_jagged_14_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
        if (var_3 < 0_f64) {
            var_4 = (var_3 * 0.5_f64);
        } else {
            var_4 = var_3;
        }
        var_5 = (((((var_2 * -3_f64) + 1.5_f64) + buffers.density_function_Cache2d_1_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize]) + (buffers.density_function_Cache2d_12_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * var_4)) * buffers.density_function_Cache2d_15_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize]);
        if (var_5 < 0_f64) {
            var_6 = (var_5 * 0.25_f64);
        } else {
            var_6 = var_5;
        }
        var_7 = clamp(((rpos3.y - -64_f64) / 384_f64), 0_f64, 1_f64);
        var_8 = buffers.minecraft_jagged_14_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
        if (var_8 < 0_f64) {
            var_9 = (var_8 * 0.5_f64);
        } else {
            var_9 = var_8;
        }
        var_10 = (((((var_7 * -3_f64) + 1.5_f64) + buffers.density_function_Cache2d_1_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize]) + (buffers.density_function_Cache2d_12_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * var_9)) * buffers.density_function_Cache2d_15_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize]);
        if (var_10 < 0_f64) {
            var_11 = (var_10 * 0.25_f64);
        } else {
            var_11 = var_10;
        }
        var_12 = buffers.minecraft_cave_layer_24_d5x49x5_output[index(p) as usize];
        var_13 = clamp(((rpos3.y - -64_f64) / 384_f64), 0_f64, 1_f64);
        var_14 = buffers.minecraft_jagged_14_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
        if (var_14 < 0_f64) {
            var_15 = (var_14 * 0.5_f64);
        } else {
            var_15 = var_14;
        }
        var_16 = (((((var_13 * -3_f64) + 1.5_f64) + buffers.density_function_Cache2d_1_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize]) + (buffers.density_function_Cache2d_12_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * var_15)) * buffers.density_function_Cache2d_15_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize]);
        if (var_16 < 0_f64) {
            var_17 = (var_16 * 0.25_f64);
        } else {
            var_17 = var_16;
        }
        var_18 = scale_caves(buffers.minecraft_spaghetti_2d_modulator_26_d5x49x5_output[index(p) as usize]);
        var_19 = clamp(((rpos3.y - -64_f64) / 384_f64), 0_f64, 1_f64);
        var_20 = (abs(((0_f64 + (8_f64 * buffers.minecraft_spaghetti_2d_elevation_29_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize])) + ((var_19 * -48_f64) + 8_f64))) + buffers.density_function_Add_27_d5x49x5_output[index(p) as usize]);
        var_21 = buffers.density_function_Multiply_31_d5x49x5_output[index(p) as usize];
        if ((var_21 >= -1000000_f64) && (var_21 < 0.03_f64)) {
            var_22 = -1000000_f64;
        } else {
            var_22 = buffers.density_function_Multiply_31_d5x49x5_output[index(p) as usize];
        }
        var_23 = ((4_f64 * var_6) + buffers.old_blended_noise_d5x49x5_output[index(p) as usize]);
        if ((var_23 >= -1000000_f64) && (var_23 < 1.5625_f64)) {
            var_24 = min(((4_f64 * var_11) + buffers.old_blended_noise_d5x49x5_output[index(p) as usize]), (5_f64 * buffers.density_function_Min_16_d5x49x5_output[index(p) as usize]));
        } else {
            var_24 = max(min(min(((4_f64 * (var_12 * var_12)) + (clamp((0.27_f64 + buffers.minecraft_cave_cheese_25_d5x49x5_output[index(p) as usize]), -1_f64, 1_f64) + clamp((1.5_f64 + (-0.64_f64 * ((4_f64 * var_17) + buffers.old_blended_noise_d5x49x5_output[index(p) as usize]))), 0_f64, 0.5_f64))), buffers.density_function_Min_16_d5x49x5_output[index(p) as usize]), (clamp(max(((var_18 * abs(minecraft_spaghetti_2d_rarity_mapped_4(p, config, buffers, auto_constants, host_consts, (rpos3 / var_18)))) + (0.083_f64 * buffers.density_function_Add_27_d5x49x5_output[index(p) as usize])), (var_20 * (var_20 * var_20))), -1_f64, 1_f64) + buffers.density_function_Multiply_18_d5x49x5_output[index(p) as usize])), var_22);
        }
        return (0.1171875_f64 + (((var * 1_f64) + 0_f64) * (-0.1171875_f64 + (-0.078125_f64 + (((var_1 * -1_f64) + 1_f64) * (0.078125_f64 + var_24))))));
    }
    
    #[inline]
    fn minecraft_spaghetti_2d_rarity_mapped_4(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_spaghetti_2d_0_octave__7)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_spaghetti_2d_1_octave__7)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn density_function_66_d128x48x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut var_1: f64;
        let mut var_2: f64;
        let mut var_3: f64;
        let mut var_4: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = clamp(((rpos3.y - -64_f64) / 24_f64), 0_f64, 1_f64);
        var_1 = clamp(((rpos3.y - 240_f64) / 16_f64), 0_f64, 1_f64);
        var_2 = clamp(((rpos3.y - -64_f64) / 384_f64), 0_f64, 1_f64);
        var_3 = ((((var_2 * -3_f64) + 1.5_f64) + buffers.density_function_Cache2d_54_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize]) * buffers.density_function_Cache2d_64_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize]);
        if (var_3 < 0_f64) {
            var_4 = (var_3 * 0.25_f64);
        } else {
            var_4 = var_3;
        }
        return (-0.390625_f64 + (0.1171875_f64 + (((var * 1_f64) + 0_f64) * (-0.1171875_f64 + (-0.078125_f64 + (((var_1 * -1_f64) + 1_f64) * (0.078125_f64 + clamp((-0.703125_f64 + (4_f64 * var_4)), -64_f64, 64_f64))))))));
    }
    
    #[inline]
    fn final_density_with_beardify_d16x384x16(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut var_1: f64;
        let mut var_2: f64;
        let mut var_3: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = clamp((0.64_f64 * interpolate(buffers.density_function_Add_0_d5x49x5_output[cornerx0y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_Add_0_d5x49x5_output[cornerx4y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_Add_0_d5x49x5_output[cornerx0y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_Add_0_d5x49x5_output[cornerx4y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_Add_0_d5x49x5_output[cornerx0y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_Add_0_d5x49x5_output[cornerx4y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_Add_0_d5x49x5_output[cornerx0y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_Add_0_d5x49x5_output[cornerx4y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], xfract4(pos3), yfract8(pos3), zfract4(pos3))), -1_f64, 1_f64);
        var_1 = ((var * 0.5_f64) - (((var * var) * var) * 0.041666666666666664_f64));
        var_2 = interpolate(buffers.density_function_RangeChoice_35_d5x49x5_output[cornerx0y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_35_d5x49x5_output[cornerx4y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_35_d5x49x5_output[cornerx0y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_35_d5x49x5_output[cornerx4y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_35_d5x49x5_output[cornerx0y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_35_d5x49x5_output[cornerx4y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_35_d5x49x5_output[cornerx0y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_35_d5x49x5_output[cornerx4y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], xfract4(pos3), yfract8(pos3), zfract4(pos3));
        if ((var_2 >= -1000000_f64) && (var_2 < 0_f64)) {
            var_3 = 64_f64;
        } else {
            var_3 = (interpolate(buffers.density_function_RangeChoice_37_d5x49x5_output[cornerx0y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_37_d5x49x5_output[cornerx4y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_37_d5x49x5_output[cornerx0y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_37_d5x49x5_output[cornerx4y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_37_d5x49x5_output[cornerx0y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_37_d5x49x5_output[cornerx4y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_37_d5x49x5_output[cornerx0y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_37_d5x49x5_output[cornerx4y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], xfract4(pos3), yfract8(pos3), zfract4(pos3)) + (1.5_f64 * max(abs(interpolate(buffers.density_function_RangeChoice_39_d5x49x5_output[cornerx0y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_39_d5x49x5_output[cornerx4y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_39_d5x49x5_output[cornerx0y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_39_d5x49x5_output[cornerx4y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_39_d5x49x5_output[cornerx0y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_39_d5x49x5_output[cornerx4y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_39_d5x49x5_output[cornerx0y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_39_d5x49x5_output[cornerx4y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], xfract4(pos3), yfract8(pos3), zfract4(pos3))), abs(interpolate(buffers.density_function_RangeChoice_41_d5x49x5_output[cornerx0y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_41_d5x49x5_output[cornerx4y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_41_d5x49x5_output[cornerx0y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_41_d5x49x5_output[cornerx4y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_41_d5x49x5_output[cornerx0y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_41_d5x49x5_output[cornerx4y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_41_d5x49x5_output[cornerx0y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_41_d5x49x5_output[cornerx4y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], xfract4(pos3), yfract8(pos3), zfract4(pos3))))));
        }
        return (min(var_1, var_3) + buffers.Beardifier_d16x384x16_output[index(p) as usize]);
    }
    
    #[inline]
    fn final_density_d16x384x16(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut var_1: f64;
        let mut var_2: f64;
        let mut var_3: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = clamp((0.64_f64 * interpolate(buffers.density_function_Add_0_d5x49x5_output[cornerx0y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_Add_0_d5x49x5_output[cornerx4y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_Add_0_d5x49x5_output[cornerx0y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_Add_0_d5x49x5_output[cornerx4y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_Add_0_d5x49x5_output[cornerx0y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_Add_0_d5x49x5_output[cornerx4y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_Add_0_d5x49x5_output[cornerx0y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_Add_0_d5x49x5_output[cornerx4y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], xfract4(pos3), yfract8(pos3), zfract4(pos3))), -1_f64, 1_f64);
        var_1 = ((var * 0.5_f64) - (((var * var) * var) * 0.041666666666666664_f64));
        var_2 = interpolate(buffers.density_function_RangeChoice_35_d5x49x5_output[cornerx0y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_35_d5x49x5_output[cornerx4y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_35_d5x49x5_output[cornerx0y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_35_d5x49x5_output[cornerx4y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_35_d5x49x5_output[cornerx0y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_35_d5x49x5_output[cornerx4y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_35_d5x49x5_output[cornerx0y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_35_d5x49x5_output[cornerx4y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], xfract4(pos3), yfract8(pos3), zfract4(pos3));
        if ((var_2 >= -1000000_f64) && (var_2 < 0_f64)) {
            var_3 = 64_f64;
        } else {
            var_3 = (interpolate(buffers.density_function_RangeChoice_37_d5x49x5_output[cornerx0y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_37_d5x49x5_output[cornerx4y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_37_d5x49x5_output[cornerx0y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_37_d5x49x5_output[cornerx4y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_37_d5x49x5_output[cornerx0y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_37_d5x49x5_output[cornerx4y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_37_d5x49x5_output[cornerx0y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_37_d5x49x5_output[cornerx4y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], xfract4(pos3), yfract8(pos3), zfract4(pos3)) + (1.5_f64 * max(abs(interpolate(buffers.density_function_RangeChoice_39_d5x49x5_output[cornerx0y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_39_d5x49x5_output[cornerx4y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_39_d5x49x5_output[cornerx0y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_39_d5x49x5_output[cornerx4y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_39_d5x49x5_output[cornerx0y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_39_d5x49x5_output[cornerx4y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_39_d5x49x5_output[cornerx0y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_39_d5x49x5_output[cornerx4y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], xfract4(pos3), yfract8(pos3), zfract4(pos3))), abs(interpolate(buffers.density_function_RangeChoice_41_d5x49x5_output[cornerx0y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_41_d5x49x5_output[cornerx4y0z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_41_d5x49x5_output[cornerx0y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_41_d5x49x5_output[cornerx4y8z0_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_41_d5x49x5_output[cornerx0y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_41_d5x49x5_output[cornerx4y0z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_41_d5x49x5_output[cornerx0y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], buffers.density_function_RangeChoice_41_d5x49x5_output[cornerx4y8z4_8(pos3, 5_i32, 49_i32, 5_i32) as usize], xfract4(pos3), yfract8(pos3), zfract4(pos3))))));
        }
        return min(var_1, var_3);
    }
    
    #[inline]
    fn preliminary_surface_level_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: i32;
        let mut var_1: i32;
        let mut var_2: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = ((floor((clamp((128_f64 + (-128_f64 * ((0.2734375_f64 * (1_f64 / buffers.density_function_Cache2d_64_d128x1x128_output[index(p) as usize])) + (-1_f64 * buffers.density_function_Cache2d_54_d128x1x128_output[index(p) as usize])))), -40_f64, 320_f64) / 8_f64)) * 8_f64) as i32);
        if (var <= -64_i32) {
            var = -64_i32;
        } else {
            var_1 = (var / 8_i32);
            var = -64_i32;
            while (var_1 >= -8_i32) {
                if (buffers.density_function_66_d128x48x128_output[preliminary_surface_index(pos3, ((var_1 - -8_i32) as i32), 8_i32, 128_i32, 48_i32, 128_i32) as usize] > 0_f64) {
                    var = (var_1 * 8_i32);
                    break;
                }
                var_1 = (var_1 - 1_i32);
            }
        }
        var_2 = (var as f64);
        return var_2;
    }
    
    pub fn Beardifier_d16x384x16_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(1_f64, 1_f64, 1_f64), dimensions: (16_i32, 384_i32, 16_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = Beardifier_d16x384x16(p, (&config), buffers, auto_constants, host_consts);
            buffers.Beardifier_d16x384x16_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_jagged_14_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1500_f64, 0_f64, 1500_f64), position_scale: Vec3::new(6000_f64, 0_f64, 6000_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_jagged_14_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_jagged_14_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn old_blended_noise_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = old_blended_noise_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.old_blended_noise_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_cave_layer_24_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 8_f64, 1_f64), position_scale: Vec3::new(4_f64, 64_f64, 4_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_cave_layer_24_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_cave_layer_24_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_cave_cheese_25_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0.6666666666666666_f64, 1_f64), position_scale: Vec3::new(4_f64, 5.333333333333333_f64, 4_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_cave_cheese_25_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_cave_cheese_25_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_spaghetti_2d_modulator_26_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(2_f64, 1_f64, 2_f64), position_scale: Vec3::new(8_f64, 8_f64, 8_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_spaghetti_2d_modulator_26_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_spaghetti_2d_modulator_26_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_spaghetti_2d_elevation_29_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_spaghetti_2d_elevation_29_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_spaghetti_2d_elevation_29_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_noodle_36_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_noodle_36_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_noodle_36_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_noodle_thickness_38_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_noodle_thickness_38_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_noodle_thickness_38_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_noodle_ridge_a_40_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(2.6666666666666665_f64, 2.6666666666666665_f64, 2.6666666666666665_f64), position_scale: Vec3::new(10.666666666666666_f64, 21.333333333333332_f64, 10.666666666666666_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_noodle_ridge_a_40_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_noodle_ridge_a_40_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_noodle_ridge_b_42_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(2.6666666666666665_f64, 2.6666666666666665_f64, 2.6666666666666665_f64), position_scale: Vec3::new(10.666666666666666_f64, 21.333333333333332_f64, 10.666666666666666_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_noodle_ridge_b_42_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_noodle_ridge_b_42_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_ore_vein_a_49_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(4_f64, 4_f64, 4_f64), position_scale: Vec3::new(16_f64, 32_f64, 16_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_ore_vein_a_49_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_ore_vein_a_49_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_ore_vein_b_51_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(4_f64, 4_f64, 4_f64), position_scale: Vec3::new(16_f64, 32_f64, 16_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_ore_vein_b_51_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_ore_vein_b_51_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_ore_veininess_53_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1.5_f64, 1.5_f64, 1.5_f64), position_scale: Vec3::new(6_f64, 12_f64, 6_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_ore_veininess_53_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_ore_veininess_53_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_cave_entrance_17_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.75_f64, 0.5_f64, 0.75_f64), position_scale: Vec3::new(3_f64, 4_f64, 3_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_cave_entrance_17_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_cave_entrance_17_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_spaghetti_3d_thickness_23_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_spaghetti_3d_thickness_23_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_spaghetti_3d_thickness_23_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_spaghetti_2d_thickness_28_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(2_f64, 1_f64, 2_f64), position_scale: Vec3::new(8_f64, 8_f64, 8_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_spaghetti_2d_thickness_28_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_spaghetti_2d_thickness_28_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_spaghetti_roughness_modulator_19_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_spaghetti_roughness_modulator_19_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_spaghetti_roughness_modulator_19_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_spaghetti_roughness_20_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_spaghetti_roughness_20_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_spaghetti_roughness_20_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_pillar_32_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(25_f64, 0.3_f64, 25_f64), position_scale: Vec3::new(100_f64, 2.4_f64, 100_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_pillar_32_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_pillar_32_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_pillar_rareness_33_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_pillar_rareness_33_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_pillar_rareness_33_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_pillar_thickness_34_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_pillar_thickness_34_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_pillar_thickness_34_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_spaghetti_3d_rarity_22_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(2_f64, 1_f64, 2_f64), position_scale: Vec3::new(8_f64, 8_f64, 8_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_spaghetti_3d_rarity_22_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_spaghetti_3d_rarity_22_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_offset_3_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.25_f64, 0_f64, 0.25_f64), position_scale: Vec3::new(1_f64, 0_f64, 1_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_offset_3_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_offset_3_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_offset_3_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.25_f64, 0_f64, 0.25_f64), position_scale: Vec3::new(1_f64, 0_f64, 1_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_offset_3_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_offset_3_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_RangeChoice_35_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_RangeChoice_35_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_RangeChoice_35_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_RangeChoice_37_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_RangeChoice_37_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_RangeChoice_37_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_RangeChoice_39_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_RangeChoice_39_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_RangeChoice_39_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_RangeChoice_41_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_RangeChoice_41_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_RangeChoice_41_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_RangeChoice_48_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_RangeChoice_48_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_RangeChoice_48_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_RangeChoice_50_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_RangeChoice_50_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_RangeChoice_50_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_RangeChoice_52_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_RangeChoice_52_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_RangeChoice_52_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_Add_27_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_Add_27_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_Add_27_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_Multiply_18_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_Multiply_18_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_Multiply_18_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_Multiply_31_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_Multiply_31_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_Multiply_31_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_Noise_21_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_Noise_21_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_Noise_21_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_ShiftedNoise_55_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_ShiftedNoise_55_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_ShiftedNoise_55_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_ShiftedNoise_58_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_ShiftedNoise_58_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_ShiftedNoise_58_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_ShiftedNoise_61_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_ShiftedNoise_61_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_ShiftedNoise_61_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_ShiftedNoise_2_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_ShiftedNoise_2_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_ShiftedNoise_2_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_ShiftedNoise_6_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_ShiftedNoise_6_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_ShiftedNoise_6_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_ShiftedNoise_9_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_ShiftedNoise_9_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_ShiftedNoise_9_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn vein_ridged_d16x384x16_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(1_f64, 1_f64, 1_f64), dimensions: (16_i32, 384_i32, 16_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = vein_ridged_d16x384x16(p, (&config), buffers, auto_constants, host_consts);
            buffers.vein_ridged_d16x384x16_output[index(p) as usize] = r;
        }
    }
    
    pub fn vein_toggle_d16x384x16_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(1_f64, 1_f64, 1_f64), dimensions: (16_i32, 384_i32, 16_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = vein_toggle_d16x384x16(p, (&config), buffers, auto_constants, host_consts);
            buffers.vein_toggle_d16x384x16_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_Min_16_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_Min_16_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_Min_16_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_overworld_ridges_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_overworld_ridges_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_overworld_ridges_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_overworld_ridges_folded_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_overworld_ridges_folded_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_overworld_ridges_folded_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_overworld_erosion_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_overworld_erosion_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_overworld_erosion_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_overworld_continents_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_overworld_continents_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_overworld_continents_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_overworld_ridges_folded_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_overworld_ridges_folded_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_overworld_ridges_folded_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_overworld_erosion_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_overworld_erosion_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_overworld_erosion_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_overworld_continents_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_overworld_continents_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_overworld_continents_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_overworld_ridges_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_overworld_ridges_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_overworld_ridges_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_Cache2d_64_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_Cache2d_64_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_Cache2d_64_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_Cache2d_54_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_Cache2d_54_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_Cache2d_54_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_Cache2d_1_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_Cache2d_1_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_Cache2d_1_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_Cache2d_12_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_Cache2d_12_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_Cache2d_12_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_Cache2d_15_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_Cache2d_15_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_Cache2d_15_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_Add_0_d5x49x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 49_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_Add_0_d5x49x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_Add_0_d5x49x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_66_d128x48x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (128_i32, 48_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_66_d128x48x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_66_d128x48x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn final_density_with_beardify_d16x384x16_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(1_f64, 1_f64, 1_f64), dimensions: (16_i32, 384_i32, 16_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = final_density_with_beardify_d16x384x16(p, (&config), buffers, auto_constants, host_consts);
            buffers.final_density_with_beardify_d16x384x16_output[index(p) as usize] = r;
        }
    }
    
    pub fn final_density_d16x384x16_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(1_f64, 1_f64, 1_f64), dimensions: (16_i32, 384_i32, 16_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = final_density_d16x384x16(p, (&config), buffers, auto_constants, host_consts);
            buffers.final_density_d16x384x16_output[index(p) as usize] = r;
        }
    }
    
    pub fn preliminary_surface_level_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = preliminary_surface_level_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.preliminary_surface_level_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
}


pub mod orchestration {
    // Auto-generated inline Rust module from RCL
    #[allow(dead_code)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(warnings)]
    #[allow(clippy::all)]
    
    use super::*;
    use super::density_function::*;
    
    #[repr(C)]
    pub struct HostConsts {
        pub base_box: BlockBox,
        pub jigsaw_array: [Jigsaw; 500],
        pub piece_array: [Piece; 500],
    }
    
    #[repr(C)]
    #[derive(Clone)]
    pub struct AutoConsts {
        pub minecraft_spaghetti_2d_0_octave__7: PerlinNoiseSampler,
        pub minecraft_spaghetti_2d_1_octave__7: PerlinNoiseSampler,
        pub minecraft_offset_0_octave__3: PerlinNoiseSampler,
        pub minecraft_offset_1_octave__3: PerlinNoiseSampler,
        pub minecraft_offset_0_octave__2: PerlinNoiseSampler,
        pub minecraft_offset_1_octave__2: PerlinNoiseSampler,
        pub minecraft_offset_0_octave__1: PerlinNoiseSampler,
        pub minecraft_offset_1_octave__1: PerlinNoiseSampler,
        pub minecraft_ridge_0_octave__7: PerlinNoiseSampler,
        pub minecraft_ridge_1_octave__7: PerlinNoiseSampler,
        pub minecraft_ridge_0_octave__6: PerlinNoiseSampler,
        pub minecraft_ridge_1_octave__6: PerlinNoiseSampler,
        pub minecraft_ridge_0_octave__5: PerlinNoiseSampler,
        pub minecraft_ridge_1_octave__5: PerlinNoiseSampler,
        pub minecraft_erosion_0_octave__9: PerlinNoiseSampler,
        pub minecraft_erosion_1_octave__9: PerlinNoiseSampler,
        pub minecraft_erosion_0_octave__8: PerlinNoiseSampler,
        pub minecraft_erosion_1_octave__8: PerlinNoiseSampler,
        pub minecraft_erosion_0_octave__6: PerlinNoiseSampler,
        pub minecraft_erosion_1_octave__6: PerlinNoiseSampler,
        pub minecraft_erosion_0_octave__5: PerlinNoiseSampler,
        pub minecraft_erosion_1_octave__5: PerlinNoiseSampler,
        pub minecraft_continentalness_0_octave__9: PerlinNoiseSampler,
        pub minecraft_continentalness_1_octave__9: PerlinNoiseSampler,
        pub minecraft_continentalness_0_octave__8: PerlinNoiseSampler,
        pub minecraft_continentalness_1_octave__8: PerlinNoiseSampler,
        pub minecraft_continentalness_0_octave__7: PerlinNoiseSampler,
        pub minecraft_continentalness_1_octave__7: PerlinNoiseSampler,
        pub minecraft_continentalness_0_octave__6: PerlinNoiseSampler,
        pub minecraft_continentalness_1_octave__6: PerlinNoiseSampler,
        pub minecraft_continentalness_0_octave__5: PerlinNoiseSampler,
        pub minecraft_continentalness_1_octave__5: PerlinNoiseSampler,
        pub minecraft_continentalness_0_octave__4: PerlinNoiseSampler,
        pub minecraft_continentalness_1_octave__4: PerlinNoiseSampler,
        pub minecraft_continentalness_0_octave__3: PerlinNoiseSampler,
        pub minecraft_continentalness_1_octave__3: PerlinNoiseSampler,
        pub minecraft_continentalness_0_octave__2: PerlinNoiseSampler,
        pub minecraft_continentalness_1_octave__2: PerlinNoiseSampler,
        pub minecraft_continentalness_0_octave__1: PerlinNoiseSampler,
        pub minecraft_continentalness_1_octave__1: PerlinNoiseSampler,
        pub minecraft_jagged_0_octave__16: PerlinNoiseSampler,
        pub minecraft_jagged_1_octave__16: PerlinNoiseSampler,
        pub minecraft_jagged_0_octave__15: PerlinNoiseSampler,
        pub minecraft_jagged_1_octave__15: PerlinNoiseSampler,
        pub minecraft_jagged_0_octave__14: PerlinNoiseSampler,
        pub minecraft_jagged_1_octave__14: PerlinNoiseSampler,
        pub minecraft_jagged_0_octave__13: PerlinNoiseSampler,
        pub minecraft_jagged_1_octave__13: PerlinNoiseSampler,
        pub minecraft_jagged_0_octave__12: PerlinNoiseSampler,
        pub minecraft_jagged_1_octave__12: PerlinNoiseSampler,
        pub minecraft_jagged_0_octave__11: PerlinNoiseSampler,
        pub minecraft_jagged_1_octave__11: PerlinNoiseSampler,
        pub minecraft_jagged_0_octave__10: PerlinNoiseSampler,
        pub minecraft_jagged_1_octave__10: PerlinNoiseSampler,
        pub minecraft_jagged_0_octave__9: PerlinNoiseSampler,
        pub minecraft_jagged_1_octave__9: PerlinNoiseSampler,
        pub minecraft_jagged_0_octave__8: PerlinNoiseSampler,
        pub minecraft_jagged_1_octave__8: PerlinNoiseSampler,
        pub minecraft_jagged_0_octave__7: PerlinNoiseSampler,
        pub minecraft_jagged_1_octave__7: PerlinNoiseSampler,
        pub minecraft_jagged_0_octave__6: PerlinNoiseSampler,
        pub minecraft_jagged_1_octave__6: PerlinNoiseSampler,
        pub minecraft_jagged_0_octave__5: PerlinNoiseSampler,
        pub minecraft_jagged_1_octave__5: PerlinNoiseSampler,
        pub minecraft_jagged_0_octave__4: PerlinNoiseSampler,
        pub minecraft_jagged_1_octave__4: PerlinNoiseSampler,
        pub minecraft_jagged_0_octave__3: PerlinNoiseSampler,
        pub minecraft_jagged_1_octave__3: PerlinNoiseSampler,
        pub minecraft_jagged_0_octave__2: PerlinNoiseSampler,
        pub minecraft_jagged_1_octave__2: PerlinNoiseSampler,
        pub minecraft_jagged_0_octave__1: PerlinNoiseSampler,
        pub minecraft_jagged_1_octave__1: PerlinNoiseSampler,
        pub base3d_perm_table: InterpolatedNoiseSampler,
        pub minecraft_spaghetti_3d_1_0_octave__7: PerlinNoiseSampler,
        pub minecraft_spaghetti_3d_1_1_octave__7: PerlinNoiseSampler,
        pub minecraft_spaghetti_3d_2_0_octave__7: PerlinNoiseSampler,
        pub minecraft_spaghetti_3d_2_1_octave__7: PerlinNoiseSampler,
        pub minecraft_cave_entrance_0_octave__7: PerlinNoiseSampler,
        pub minecraft_cave_entrance_1_octave__7: PerlinNoiseSampler,
        pub minecraft_cave_entrance_0_octave__6: PerlinNoiseSampler,
        pub minecraft_cave_entrance_1_octave__6: PerlinNoiseSampler,
        pub minecraft_cave_entrance_0_octave__5: PerlinNoiseSampler,
        pub minecraft_cave_entrance_1_octave__5: PerlinNoiseSampler,
        pub minecraft_spaghetti_roughness_modulator_0_octave__8: PerlinNoiseSampler,
        pub minecraft_spaghetti_roughness_modulator_1_octave__8: PerlinNoiseSampler,
        pub minecraft_spaghetti_roughness_0_octave__5: PerlinNoiseSampler,
        pub minecraft_spaghetti_roughness_1_octave__5: PerlinNoiseSampler,
        pub minecraft_spaghetti_3d_rarity_0_octave__11: PerlinNoiseSampler,
        pub minecraft_spaghetti_3d_rarity_1_octave__11: PerlinNoiseSampler,
        pub minecraft_spaghetti_3d_thickness_0_octave__8: PerlinNoiseSampler,
        pub minecraft_spaghetti_3d_thickness_1_octave__8: PerlinNoiseSampler,
        pub minecraft_cave_layer_0_octave__8: PerlinNoiseSampler,
        pub minecraft_cave_layer_1_octave__8: PerlinNoiseSampler,
        pub minecraft_cave_cheese_0_octave__8: PerlinNoiseSampler,
        pub minecraft_cave_cheese_1_octave__8: PerlinNoiseSampler,
        pub minecraft_cave_cheese_0_octave__7: PerlinNoiseSampler,
        pub minecraft_cave_cheese_1_octave__7: PerlinNoiseSampler,
        pub minecraft_cave_cheese_0_octave__6: PerlinNoiseSampler,
        pub minecraft_cave_cheese_1_octave__6: PerlinNoiseSampler,
        pub minecraft_cave_cheese_0_octave__5: PerlinNoiseSampler,
        pub minecraft_cave_cheese_1_octave__5: PerlinNoiseSampler,
        pub minecraft_cave_cheese_0_octave__4: PerlinNoiseSampler,
        pub minecraft_cave_cheese_1_octave__4: PerlinNoiseSampler,
        pub minecraft_cave_cheese_0_octave__3: PerlinNoiseSampler,
        pub minecraft_cave_cheese_1_octave__3: PerlinNoiseSampler,
        pub minecraft_cave_cheese_0_octave__1: PerlinNoiseSampler,
        pub minecraft_cave_cheese_1_octave__1: PerlinNoiseSampler,
        pub minecraft_spaghetti_2d_modulator_0_octave__11: PerlinNoiseSampler,
        pub minecraft_spaghetti_2d_modulator_1_octave__11: PerlinNoiseSampler,
        pub minecraft_spaghetti_2d_thickness_0_octave__11: PerlinNoiseSampler,
        pub minecraft_spaghetti_2d_thickness_1_octave__11: PerlinNoiseSampler,
        pub minecraft_spaghetti_2d_elevation_0_octave__8: PerlinNoiseSampler,
        pub minecraft_spaghetti_2d_elevation_1_octave__8: PerlinNoiseSampler,
        pub minecraft_pillar_0_octave__7: PerlinNoiseSampler,
        pub minecraft_pillar_1_octave__7: PerlinNoiseSampler,
        pub minecraft_pillar_0_octave__6: PerlinNoiseSampler,
        pub minecraft_pillar_1_octave__6: PerlinNoiseSampler,
        pub minecraft_pillar_rareness_0_octave__8: PerlinNoiseSampler,
        pub minecraft_pillar_rareness_1_octave__8: PerlinNoiseSampler,
        pub minecraft_pillar_thickness_0_octave__8: PerlinNoiseSampler,
        pub minecraft_pillar_thickness_1_octave__8: PerlinNoiseSampler,
        pub minecraft_noodle_0_octave__8: PerlinNoiseSampler,
        pub minecraft_noodle_1_octave__8: PerlinNoiseSampler,
        pub minecraft_noodle_thickness_0_octave__8: PerlinNoiseSampler,
        pub minecraft_noodle_thickness_1_octave__8: PerlinNoiseSampler,
        pub minecraft_noodle_ridge_a_0_octave__7: PerlinNoiseSampler,
        pub minecraft_noodle_ridge_a_1_octave__7: PerlinNoiseSampler,
        pub minecraft_noodle_ridge_b_0_octave__7: PerlinNoiseSampler,
        pub minecraft_noodle_ridge_b_1_octave__7: PerlinNoiseSampler,
        pub minecraft_ore_vein_a_0_octave__7: PerlinNoiseSampler,
        pub minecraft_ore_vein_a_1_octave__7: PerlinNoiseSampler,
        pub minecraft_ore_vein_b_0_octave__7: PerlinNoiseSampler,
        pub minecraft_ore_vein_b_1_octave__7: PerlinNoiseSampler,
        pub minecraft_ore_veininess_0_octave__8: PerlinNoiseSampler,
        pub minecraft_ore_veininess_1_octave__8: PerlinNoiseSampler,
    }
    
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct InternalBuffers {
        pub final_density_with_beardify_d16x384x16_output: [f64; 98304],
        pub final_density_d16x384x16_output: [f64; 98304],
        pub vein_ridged_d16x384x16_output: [f64; 98304],
        pub vein_toggle_d16x384x16_output: [f64; 98304],
        pub preliminary_surface_level_d128x1x128_output: [f64; 16384],
        pub density_function_Add_0_d5x49x5_output: [f64; 1225],
        pub density_function_RangeChoice_35_d5x49x5_output: [f64; 1225],
        pub density_function_RangeChoice_37_d5x49x5_output: [f64; 1225],
        pub density_function_RangeChoice_39_d5x49x5_output: [f64; 1225],
        pub density_function_RangeChoice_41_d5x49x5_output: [f64; 1225],
        pub Beardifier_d16x384x16_output: [f64; 98304],
        pub density_function_RangeChoice_48_d5x49x5_output: [f64; 1225],
        pub density_function_RangeChoice_50_d5x49x5_output: [f64; 1225],
        pub density_function_RangeChoice_52_d5x49x5_output: [f64; 1225],
        pub density_function_66_d128x48x128_output: [f64; 786432],
        pub density_function_Cache2d_64_d128x1x128_output: [f64; 16384],
        pub density_function_Cache2d_54_d128x1x128_output: [f64; 16384],
        pub density_function_Cache2d_1_d5x1x5_output: [f64; 25],
        pub density_function_Cache2d_12_d5x1x5_output: [f64; 25],
        pub minecraft_jagged_14_d5x1x5_output: [f64; 25],
        pub density_function_Cache2d_15_d5x1x5_output: [f64; 25],
        pub old_blended_noise_d5x49x5_output: [f64; 1225],
        pub density_function_Min_16_d5x49x5_output: [f64; 1225],
        pub minecraft_cave_layer_24_d5x49x5_output: [f64; 1225],
        pub minecraft_cave_cheese_25_d5x49x5_output: [f64; 1225],
        pub minecraft_spaghetti_2d_modulator_26_d5x49x5_output: [f64; 1225],
        pub density_function_Add_27_d5x49x5_output: [f64; 1225],
        pub minecraft_spaghetti_2d_elevation_29_d5x1x5_output: [f64; 25],
        pub density_function_Multiply_18_d5x49x5_output: [f64; 1225],
        pub density_function_Multiply_31_d5x49x5_output: [f64; 1225],
        pub minecraft_noodle_36_d5x49x5_output: [f64; 1225],
        pub minecraft_noodle_thickness_38_d5x49x5_output: [f64; 1225],
        pub minecraft_noodle_ridge_a_40_d5x49x5_output: [f64; 1225],
        pub minecraft_noodle_ridge_b_42_d5x49x5_output: [f64; 1225],
        pub minecraft_ore_vein_a_49_d5x49x5_output: [f64; 1225],
        pub minecraft_ore_vein_b_51_d5x49x5_output: [f64; 1225],
        pub minecraft_ore_veininess_53_d5x49x5_output: [f64; 1225],
        pub minecraft_overworld_ridges_d128x1x128_output: [f64; 16384],
        pub minecraft_overworld_ridges_folded_d128x1x128_output: [f64; 16384],
        pub minecraft_overworld_erosion_d128x1x128_output: [f64; 16384],
        pub minecraft_overworld_continents_d128x1x128_output: [f64; 16384],
        pub minecraft_overworld_ridges_folded_d5x1x5_output: [f64; 25],
        pub minecraft_overworld_erosion_d5x1x5_output: [f64; 25],
        pub minecraft_overworld_continents_d5x1x5_output: [f64; 25],
        pub minecraft_overworld_ridges_d5x1x5_output: [f64; 25],
        pub minecraft_cave_entrance_17_d5x49x5_output: [f64; 1225],
        pub density_function_Noise_21_d5x49x5_output: [f64; 1225],
        pub minecraft_spaghetti_3d_thickness_23_d5x49x5_output: [f64; 1225],
        pub minecraft_spaghetti_2d_thickness_28_d5x49x5_output: [f64; 1225],
        pub minecraft_spaghetti_roughness_modulator_19_d5x49x5_output: [f64; 1225],
        pub minecraft_spaghetti_roughness_20_d5x49x5_output: [f64; 1225],
        pub minecraft_pillar_32_d5x49x5_output: [f64; 1225],
        pub minecraft_pillar_rareness_33_d5x49x5_output: [f64; 1225],
        pub minecraft_pillar_thickness_34_d5x49x5_output: [f64; 1225],
        pub density_function_ShiftedNoise_55_d128x1x128_output: [f64; 16384],
        pub density_function_ShiftedNoise_58_d128x1x128_output: [f64; 16384],
        pub density_function_ShiftedNoise_61_d128x1x128_output: [f64; 16384],
        pub density_function_ShiftedNoise_2_d5x1x5_output: [f64; 25],
        pub density_function_ShiftedNoise_6_d5x1x5_output: [f64; 25],
        pub density_function_ShiftedNoise_9_d5x1x5_output: [f64; 25],
        pub minecraft_spaghetti_3d_rarity_22_d5x49x5_output: [f64; 1225],
        pub minecraft_offset_3_d128x1x128_output: [f64; 16384],
        pub minecraft_offset_3_d5x1x5_output: [f64; 25],
    }
    
    #[derive(Clone)]
    pub struct Orchestration {
        pub buffers: Box<InternalBuffers>,
        pub auto_constants: Box<AutoConsts>,
    }
    
    fn make_auto_consts(seed: i64, auto_consts: &mut AutoConsts) {
        auto_consts.minecraft_spaghetti_2d_0_octave__7 = make_permutation_table(seed, 3779800575929599095_i64, 3026987529072654797_i64, 0_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_spaghetti_2d_1_octave__7 = make_permutation_table(seed, 3779800575929599095_i64, 3026987529072654797_i64, 1_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_offset_0_octave__3 = make_permutation_table(seed, 577895406318539652_i64, 4557074653038767061_i64, 0_i64, 6040343492819601496_i64, -4840870798921671874_i64);
        auto_consts.minecraft_offset_1_octave__3 = make_permutation_table(seed, 577895406318539652_i64, 4557074653038767061_i64, 1_i64, 6040343492819601496_i64, -4840870798921671874_i64);
        auto_consts.minecraft_offset_0_octave__2 = make_permutation_table(seed, 577895406318539652_i64, 4557074653038767061_i64, 0_i64, -5430693012044290181_i64, 162122330481997252_i64);
        auto_consts.minecraft_offset_1_octave__2 = make_permutation_table(seed, 577895406318539652_i64, 4557074653038767061_i64, 1_i64, -5430693012044290181_i64, 162122330481997252_i64);
        auto_consts.minecraft_offset_0_octave__1 = make_permutation_table(seed, 577895406318539652_i64, 4557074653038767061_i64, 0_i64, -2307493697404144120_i64, -5064731985740145495_i64);
        auto_consts.minecraft_offset_1_octave__1 = make_permutation_table(seed, 577895406318539652_i64, 4557074653038767061_i64, 1_i64, -2307493697404144120_i64, -5064731985740145495_i64);
        auto_consts.minecraft_ridge_0_octave__7 = make_permutation_table(seed, -1168420988404094156_i64, 2012804684704589034_i64, 0_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_ridge_1_octave__7 = make_permutation_table(seed, -1168420988404094156_i64, 2012804684704589034_i64, 1_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_ridge_0_octave__6 = make_permutation_table(seed, -1168420988404094156_i64, 2012804684704589034_i64, 0_i64, -1937505727046359452_i64, 6887262389667105861_i64);
        auto_consts.minecraft_ridge_1_octave__6 = make_permutation_table(seed, -1168420988404094156_i64, 2012804684704589034_i64, 1_i64, -1937505727046359452_i64, 6887262389667105861_i64);
        auto_consts.minecraft_ridge_0_octave__5 = make_permutation_table(seed, -1168420988404094156_i64, 2012804684704589034_i64, 0_i64, 7888980432583755018_i64, 3328269827262531447_i64);
        auto_consts.minecraft_ridge_1_octave__5 = make_permutation_table(seed, -1168420988404094156_i64, 2012804684704589034_i64, 1_i64, 7888980432583755018_i64, 3328269827262531447_i64);
        auto_consts.minecraft_erosion_0_octave__9 = make_permutation_table(seed, -3448470997537165352_i64, 5157273775208757888_i64, 0_i64, 589938935082149425_i64, 5662732952352513153_i64);
        auto_consts.minecraft_erosion_1_octave__9 = make_permutation_table(seed, -3448470997537165352_i64, 5157273775208757888_i64, 1_i64, 589938935082149425_i64, 5662732952352513153_i64);
        auto_consts.minecraft_erosion_0_octave__8 = make_permutation_table(seed, -3448470997537165352_i64, 5157273775208757888_i64, 0_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_erosion_1_octave__8 = make_permutation_table(seed, -3448470997537165352_i64, 5157273775208757888_i64, 1_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_erosion_0_octave__6 = make_permutation_table(seed, -3448470997537165352_i64, 5157273775208757888_i64, 0_i64, -1937505727046359452_i64, 6887262389667105861_i64);
        auto_consts.minecraft_erosion_1_octave__6 = make_permutation_table(seed, -3448470997537165352_i64, 5157273775208757888_i64, 1_i64, -1937505727046359452_i64, 6887262389667105861_i64);
        auto_consts.minecraft_erosion_0_octave__5 = make_permutation_table(seed, -3448470997537165352_i64, 5157273775208757888_i64, 0_i64, 7888980432583755018_i64, 3328269827262531447_i64);
        auto_consts.minecraft_erosion_1_octave__5 = make_permutation_table(seed, -3448470997537165352_i64, 5157273775208757888_i64, 1_i64, 7888980432583755018_i64, 3328269827262531447_i64);
        auto_consts.minecraft_continentalness_0_octave__9 = make_permutation_table(seed, -8968799236159986078_i64, -5789877984865097555_i64, 0_i64, 589938935082149425_i64, 5662732952352513153_i64);
        auto_consts.minecraft_continentalness_1_octave__9 = make_permutation_table(seed, -8968799236159986078_i64, -5789877984865097555_i64, 1_i64, 589938935082149425_i64, 5662732952352513153_i64);
        auto_consts.minecraft_continentalness_0_octave__8 = make_permutation_table(seed, -8968799236159986078_i64, -5789877984865097555_i64, 0_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_continentalness_1_octave__8 = make_permutation_table(seed, -8968799236159986078_i64, -5789877984865097555_i64, 1_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_continentalness_0_octave__7 = make_permutation_table(seed, -8968799236159986078_i64, -5789877984865097555_i64, 0_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_continentalness_1_octave__7 = make_permutation_table(seed, -8968799236159986078_i64, -5789877984865097555_i64, 1_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_continentalness_0_octave__6 = make_permutation_table(seed, -8968799236159986078_i64, -5789877984865097555_i64, 0_i64, -1937505727046359452_i64, 6887262389667105861_i64);
        auto_consts.minecraft_continentalness_1_octave__6 = make_permutation_table(seed, -8968799236159986078_i64, -5789877984865097555_i64, 1_i64, -1937505727046359452_i64, 6887262389667105861_i64);
        auto_consts.minecraft_continentalness_0_octave__5 = make_permutation_table(seed, -8968799236159986078_i64, -5789877984865097555_i64, 0_i64, 7888980432583755018_i64, 3328269827262531447_i64);
        auto_consts.minecraft_continentalness_1_octave__5 = make_permutation_table(seed, -8968799236159986078_i64, -5789877984865097555_i64, 1_i64, 7888980432583755018_i64, 3328269827262531447_i64);
        auto_consts.minecraft_continentalness_0_octave__4 = make_permutation_table(seed, -8968799236159986078_i64, -5789877984865097555_i64, 0_i64, -4787091969620723870_i64, -4579290196374760307_i64);
        auto_consts.minecraft_continentalness_1_octave__4 = make_permutation_table(seed, -8968799236159986078_i64, -5789877984865097555_i64, 1_i64, -4787091969620723870_i64, -4579290196374760307_i64);
        auto_consts.minecraft_continentalness_0_octave__3 = make_permutation_table(seed, -8968799236159986078_i64, -5789877984865097555_i64, 0_i64, 6040343492819601496_i64, -4840870798921671874_i64);
        auto_consts.minecraft_continentalness_1_octave__3 = make_permutation_table(seed, -8968799236159986078_i64, -5789877984865097555_i64, 1_i64, 6040343492819601496_i64, -4840870798921671874_i64);
        auto_consts.minecraft_continentalness_0_octave__2 = make_permutation_table(seed, -8968799236159986078_i64, -5789877984865097555_i64, 0_i64, -5430693012044290181_i64, 162122330481997252_i64);
        auto_consts.minecraft_continentalness_1_octave__2 = make_permutation_table(seed, -8968799236159986078_i64, -5789877984865097555_i64, 1_i64, -5430693012044290181_i64, 162122330481997252_i64);
        auto_consts.minecraft_continentalness_0_octave__1 = make_permutation_table(seed, -8968799236159986078_i64, -5789877984865097555_i64, 0_i64, -2307493697404144120_i64, -5064731985740145495_i64);
        auto_consts.minecraft_continentalness_1_octave__1 = make_permutation_table(seed, -8968799236159986078_i64, -5789877984865097555_i64, 1_i64, -2307493697404144120_i64, -5064731985740145495_i64);
        auto_consts.minecraft_jagged_0_octave__16 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, -4173781914402293358_i64, -7690085512374011798_i64);
        auto_consts.minecraft_jagged_1_octave__16 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, -4173781914402293358_i64, -7690085512374011798_i64);
        auto_consts.minecraft_jagged_0_octave__15 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, 9146326347560021237_i64, -955544588430498797_i64);
        auto_consts.minecraft_jagged_1_octave__15 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, 9146326347560021237_i64, -955544588430498797_i64);
        auto_consts.minecraft_jagged_0_octave__14 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, -288186606250610412_i64, -5235201928609399341_i64);
        auto_consts.minecraft_jagged_1_octave__14 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, -288186606250610412_i64, -5235201928609399341_i64);
        auto_consts.minecraft_jagged_0_octave__13 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, -3315623468378726710_i64, -2582188670566161829_i64);
        auto_consts.minecraft_jagged_1_octave__13 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, -3315623468378726710_i64, -2582188670566161829_i64);
        auto_consts.minecraft_jagged_0_octave__12 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, -5649521212934510990_i64, 8900461776529241512_i64);
        auto_consts.minecraft_jagged_1_octave__12 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, -5649521212934510990_i64, 8900461776529241512_i64);
        auto_consts.minecraft_jagged_0_octave__11 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, 1141530288128540355_i64, 8405022147954297016_i64);
        auto_consts.minecraft_jagged_1_octave__11 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, 1141530288128540355_i64, 8405022147954297016_i64);
        auto_consts.minecraft_jagged_0_octave__10 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, 3950544105335881394_i64, 6623051330073944938_i64);
        auto_consts.minecraft_jagged_1_octave__10 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, 3950544105335881394_i64, 6623051330073944938_i64);
        auto_consts.minecraft_jagged_0_octave__9 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, 589938935082149425_i64, 5662732952352513153_i64);
        auto_consts.minecraft_jagged_1_octave__9 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, 589938935082149425_i64, 5662732952352513153_i64);
        auto_consts.minecraft_jagged_0_octave__8 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_jagged_1_octave__8 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_jagged_0_octave__7 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_jagged_1_octave__7 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_jagged_0_octave__6 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, -1937505727046359452_i64, 6887262389667105861_i64);
        auto_consts.minecraft_jagged_1_octave__6 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, -1937505727046359452_i64, 6887262389667105861_i64);
        auto_consts.minecraft_jagged_0_octave__5 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, 7888980432583755018_i64, 3328269827262531447_i64);
        auto_consts.minecraft_jagged_1_octave__5 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, 7888980432583755018_i64, 3328269827262531447_i64);
        auto_consts.minecraft_jagged_0_octave__4 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, -4787091969620723870_i64, -4579290196374760307_i64);
        auto_consts.minecraft_jagged_1_octave__4 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, -4787091969620723870_i64, -4579290196374760307_i64);
        auto_consts.minecraft_jagged_0_octave__3 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, 6040343492819601496_i64, -4840870798921671874_i64);
        auto_consts.minecraft_jagged_1_octave__3 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, 6040343492819601496_i64, -4840870798921671874_i64);
        auto_consts.minecraft_jagged_0_octave__2 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, -5430693012044290181_i64, 162122330481997252_i64);
        auto_consts.minecraft_jagged_1_octave__2 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, -5430693012044290181_i64, 162122330481997252_i64);
        auto_consts.minecraft_jagged_0_octave__1 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, -2307493697404144120_i64, -5064731985740145495_i64);
        auto_consts.minecraft_jagged_1_octave__1 = make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, -2307493697404144120_i64, -5064731985740145495_i64);
        auto_consts.base3d_perm_table = make_base3d_perm_table(seed);
        auto_consts.minecraft_spaghetti_3d_1_0_octave__7 = make_permutation_table(seed, -6555764053392795584_i64, -8984782917441058889_i64, 0_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_spaghetti_3d_1_1_octave__7 = make_permutation_table(seed, -6555764053392795584_i64, -8984782917441058889_i64, 1_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_spaghetti_3d_2_0_octave__7 = make_permutation_table(seed, -5069304462110044433_i64, -3018309153777972541_i64, 0_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_spaghetti_3d_2_1_octave__7 = make_permutation_table(seed, -5069304462110044433_i64, -3018309153777972541_i64, 1_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_cave_entrance_0_octave__7 = make_permutation_table(seed, -1080712077951264155_i64, -4819730529638740320_i64, 0_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_cave_entrance_1_octave__7 = make_permutation_table(seed, -1080712077951264155_i64, -4819730529638740320_i64, 1_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_cave_entrance_0_octave__6 = make_permutation_table(seed, -1080712077951264155_i64, -4819730529638740320_i64, 0_i64, -1937505727046359452_i64, 6887262389667105861_i64);
        auto_consts.minecraft_cave_entrance_1_octave__6 = make_permutation_table(seed, -1080712077951264155_i64, -4819730529638740320_i64, 1_i64, -1937505727046359452_i64, 6887262389667105861_i64);
        auto_consts.minecraft_cave_entrance_0_octave__5 = make_permutation_table(seed, -1080712077951264155_i64, -4819730529638740320_i64, 0_i64, 7888980432583755018_i64, 3328269827262531447_i64);
        auto_consts.minecraft_cave_entrance_1_octave__5 = make_permutation_table(seed, -1080712077951264155_i64, -4819730529638740320_i64, 1_i64, 7888980432583755018_i64, 3328269827262531447_i64);
        auto_consts.minecraft_spaghetti_roughness_modulator_0_octave__8 = make_permutation_table(seed, -2192259254118822230_i64, -3833182080813228029_i64, 0_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_spaghetti_roughness_modulator_1_octave__8 = make_permutation_table(seed, -2192259254118822230_i64, -3833182080813228029_i64, 1_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_spaghetti_roughness_0_octave__5 = make_permutation_table(seed, 8857832787093054865_i64, -3128414762446264387_i64, 0_i64, 7888980432583755018_i64, 3328269827262531447_i64);
        auto_consts.minecraft_spaghetti_roughness_1_octave__5 = make_permutation_table(seed, 8857832787093054865_i64, -3128414762446264387_i64, 1_i64, 7888980432583755018_i64, 3328269827262531447_i64);
        auto_consts.minecraft_spaghetti_3d_rarity_0_octave__11 = make_permutation_table(seed, -1276910366184201159_i64, -4736147208561791700_i64, 0_i64, 1141530288128540355_i64, 8405022147954297016_i64);
        auto_consts.minecraft_spaghetti_3d_rarity_1_octave__11 = make_permutation_table(seed, -1276910366184201159_i64, -4736147208561791700_i64, 1_i64, 1141530288128540355_i64, 8405022147954297016_i64);
        auto_consts.minecraft_spaghetti_3d_thickness_0_octave__8 = make_permutation_table(seed, -8539353618243048665_i64, -859009563837213269_i64, 0_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_spaghetti_3d_thickness_1_octave__8 = make_permutation_table(seed, -8539353618243048665_i64, -859009563837213269_i64, 1_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_cave_layer_0_octave__8 = make_permutation_table(seed, 5620043451534219907_i64, 6506268541312725510_i64, 0_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_cave_layer_1_octave__8 = make_permutation_table(seed, 5620043451534219907_i64, 6506268541312725510_i64, 1_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_cave_cheese_0_octave__8 = make_permutation_table(seed, -5667488503710438832_i64, 6029128391830109216_i64, 0_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_cave_cheese_1_octave__8 = make_permutation_table(seed, -5667488503710438832_i64, 6029128391830109216_i64, 1_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_cave_cheese_0_octave__7 = make_permutation_table(seed, -5667488503710438832_i64, 6029128391830109216_i64, 0_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_cave_cheese_1_octave__7 = make_permutation_table(seed, -5667488503710438832_i64, 6029128391830109216_i64, 1_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_cave_cheese_0_octave__6 = make_permutation_table(seed, -5667488503710438832_i64, 6029128391830109216_i64, 0_i64, -1937505727046359452_i64, 6887262389667105861_i64);
        auto_consts.minecraft_cave_cheese_1_octave__6 = make_permutation_table(seed, -5667488503710438832_i64, 6029128391830109216_i64, 1_i64, -1937505727046359452_i64, 6887262389667105861_i64);
        auto_consts.minecraft_cave_cheese_0_octave__5 = make_permutation_table(seed, -5667488503710438832_i64, 6029128391830109216_i64, 0_i64, 7888980432583755018_i64, 3328269827262531447_i64);
        auto_consts.minecraft_cave_cheese_1_octave__5 = make_permutation_table(seed, -5667488503710438832_i64, 6029128391830109216_i64, 1_i64, 7888980432583755018_i64, 3328269827262531447_i64);
        auto_consts.minecraft_cave_cheese_0_octave__4 = make_permutation_table(seed, -5667488503710438832_i64, 6029128391830109216_i64, 0_i64, -4787091969620723870_i64, -4579290196374760307_i64);
        auto_consts.minecraft_cave_cheese_1_octave__4 = make_permutation_table(seed, -5667488503710438832_i64, 6029128391830109216_i64, 1_i64, -4787091969620723870_i64, -4579290196374760307_i64);
        auto_consts.minecraft_cave_cheese_0_octave__3 = make_permutation_table(seed, -5667488503710438832_i64, 6029128391830109216_i64, 0_i64, 6040343492819601496_i64, -4840870798921671874_i64);
        auto_consts.minecraft_cave_cheese_1_octave__3 = make_permutation_table(seed, -5667488503710438832_i64, 6029128391830109216_i64, 1_i64, 6040343492819601496_i64, -4840870798921671874_i64);
        auto_consts.minecraft_cave_cheese_0_octave__1 = make_permutation_table(seed, -5667488503710438832_i64, 6029128391830109216_i64, 0_i64, -2307493697404144120_i64, -5064731985740145495_i64);
        auto_consts.minecraft_cave_cheese_1_octave__1 = make_permutation_table(seed, -5667488503710438832_i64, 6029128391830109216_i64, 1_i64, -2307493697404144120_i64, -5064731985740145495_i64);
        auto_consts.minecraft_spaghetti_2d_modulator_0_octave__11 = make_permutation_table(seed, -7646988369600686814_i64, 158433782627262445_i64, 0_i64, 1141530288128540355_i64, 8405022147954297016_i64);
        auto_consts.minecraft_spaghetti_2d_modulator_1_octave__11 = make_permutation_table(seed, -7646988369600686814_i64, 158433782627262445_i64, 1_i64, 1141530288128540355_i64, 8405022147954297016_i64);
        auto_consts.minecraft_spaghetti_2d_thickness_0_octave__11 = make_permutation_table(seed, 4027848931106223304_i64, -1882105290866315588_i64, 0_i64, 1141530288128540355_i64, 8405022147954297016_i64);
        auto_consts.minecraft_spaghetti_2d_thickness_1_octave__11 = make_permutation_table(seed, 4027848931106223304_i64, -1882105290866315588_i64, 1_i64, 1141530288128540355_i64, 8405022147954297016_i64);
        auto_consts.minecraft_spaghetti_2d_elevation_0_octave__8 = make_permutation_table(seed, 3016672669024775629_i64, 6607299245856183467_i64, 0_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_spaghetti_2d_elevation_1_octave__8 = make_permutation_table(seed, 3016672669024775629_i64, 6607299245856183467_i64, 1_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_pillar_0_octave__7 = make_permutation_table(seed, -3107768430481484985_i64, -3695634140662683108_i64, 0_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_pillar_1_octave__7 = make_permutation_table(seed, -3107768430481484985_i64, -3695634140662683108_i64, 1_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_pillar_0_octave__6 = make_permutation_table(seed, -3107768430481484985_i64, -3695634140662683108_i64, 0_i64, -1937505727046359452_i64, 6887262389667105861_i64);
        auto_consts.minecraft_pillar_1_octave__6 = make_permutation_table(seed, -3107768430481484985_i64, -3695634140662683108_i64, 1_i64, -1937505727046359452_i64, 6887262389667105861_i64);
        auto_consts.minecraft_pillar_rareness_0_octave__8 = make_permutation_table(seed, -8224298701182786352_i64, 325922074234395002_i64, 0_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_pillar_rareness_1_octave__8 = make_permutation_table(seed, -8224298701182786352_i64, 325922074234395002_i64, 1_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_pillar_thickness_0_octave__8 = make_permutation_table(seed, 1884979804952342669_i64, -4270642750870317428_i64, 0_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_pillar_thickness_1_octave__8 = make_permutation_table(seed, 1884979804952342669_i64, -4270642750870317428_i64, 1_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_noodle_0_octave__8 = make_permutation_table(seed, -3297513252137212853_i64, 7399974921363027187_i64, 0_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_noodle_1_octave__8 = make_permutation_table(seed, -3297513252137212853_i64, 7399974921363027187_i64, 1_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_noodle_thickness_0_octave__8 = make_permutation_table(seed, 3433040058802586130_i64, 6882284535972731025_i64, 0_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_noodle_thickness_1_octave__8 = make_permutation_table(seed, 3433040058802586130_i64, 6882284535972731025_i64, 1_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_noodle_ridge_a_0_octave__7 = make_permutation_table(seed, -8784505538917474266_i64, 5361047657610944607_i64, 0_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_noodle_ridge_a_1_octave__7 = make_permutation_table(seed, -8784505538917474266_i64, 5361047657610944607_i64, 1_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_noodle_ridge_b_0_octave__7 = make_permutation_table(seed, -1503310247173955951_i64, 224869257309605468_i64, 0_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_noodle_ridge_b_1_octave__7 = make_permutation_table(seed, -1503310247173955951_i64, 224869257309605468_i64, 1_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_ore_vein_a_0_octave__7 = make_permutation_table(seed, 5537411705652647497_i64, -3614632279681731091_i64, 0_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_ore_vein_a_1_octave__7 = make_permutation_table(seed, 5537411705652647497_i64, -3614632279681731091_i64, 1_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_ore_vein_b_0_octave__7 = make_permutation_table(seed, 7720896042651600585_i64, -5906612891113836863_i64, 0_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_ore_vein_b_1_octave__7 = make_permutation_table(seed, 7720896042651600585_i64, -5906612891113836863_i64, 1_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_ore_veininess_0_octave__8 = make_permutation_table(seed, 7748099570268139889_i64, -2846361830251817436_i64, 0_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_ore_veininess_1_octave__8 = make_permutation_table(seed, 7748099570268139889_i64, -2846361830251817436_i64, 1_i64, 1078206144088113246_i64, 5239585857299060288_i64);
    }
    
    impl Orchestration {
        pub fn new(seed: i64) -> Self {
            let mut buffers: Box<InternalBuffers> = make_boxed::<InternalBuffers>();
            let mut auto_constants: Box<AutoConsts> = make_boxed::<AutoConsts>();
            let _init: () = make_auto_consts(seed, (&mut auto_constants));
            return Self { buffers: buffers, auto_constants: auto_constants };
        }
        
        pub fn orchestrate_final_density_with_beardify(&mut self, origin: Vec3, host_consts: &HostConsts) -> &[f64; 98304] {
            let _kernel_call_0_0: () = Beardifier_d16x384x16_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_1: () = minecraft_jagged_14_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_2: () = old_blended_noise_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_3: () = minecraft_cave_layer_24_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_4: () = minecraft_cave_cheese_25_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_5: () = minecraft_spaghetti_2d_modulator_26_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_6: () = minecraft_spaghetti_2d_elevation_29_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_7: () = minecraft_noodle_36_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_8: () = minecraft_noodle_thickness_38_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_9: () = minecraft_noodle_ridge_a_40_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_10: () = minecraft_noodle_ridge_b_42_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_11: () = minecraft_cave_entrance_17_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_12: () = minecraft_spaghetti_3d_thickness_23_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_13: () = minecraft_spaghetti_2d_thickness_28_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_14: () = minecraft_spaghetti_roughness_modulator_19_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_15: () = minecraft_spaghetti_roughness_20_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_16: () = minecraft_pillar_32_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_17: () = minecraft_pillar_rareness_33_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_18: () = minecraft_pillar_thickness_34_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_19: () = minecraft_spaghetti_3d_rarity_22_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_20: () = minecraft_offset_3_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_0: () = density_function_RangeChoice_35_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_1: () = density_function_RangeChoice_37_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_2: () = density_function_RangeChoice_39_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_3: () = density_function_RangeChoice_41_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_4: () = density_function_Add_27_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_5: () = density_function_Multiply_18_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_6: () = density_function_Multiply_31_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_7: () = density_function_Noise_21_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_8: () = density_function_ShiftedNoise_2_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_9: () = density_function_ShiftedNoise_6_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_10: () = density_function_ShiftedNoise_9_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_0: () = density_function_Min_16_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_1: () = minecraft_overworld_ridges_folded_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_2: () = minecraft_overworld_erosion_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_3: () = minecraft_overworld_continents_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_4: () = minecraft_overworld_ridges_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_3_0: () = density_function_Cache2d_1_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_3_1: () = density_function_Cache2d_12_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_3_2: () = density_function_Cache2d_15_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_4_0: () = density_function_Add_0_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_5_0: () = final_density_with_beardify_d16x384x16_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            return (&self.buffers.final_density_with_beardify_d16x384x16_output);
        }
        
        pub fn orchestrate_final_density(&mut self, origin: Vec3, host_consts: &HostConsts) -> &[f64; 98304] {
            let _kernel_call_0_0: () = minecraft_jagged_14_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_1: () = old_blended_noise_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_2: () = minecraft_cave_layer_24_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_3: () = minecraft_cave_cheese_25_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_4: () = minecraft_spaghetti_2d_modulator_26_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_5: () = minecraft_spaghetti_2d_elevation_29_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_6: () = minecraft_noodle_36_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_7: () = minecraft_noodle_thickness_38_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_8: () = minecraft_noodle_ridge_a_40_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_9: () = minecraft_noodle_ridge_b_42_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_10: () = minecraft_cave_entrance_17_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_11: () = minecraft_spaghetti_3d_thickness_23_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_12: () = minecraft_spaghetti_2d_thickness_28_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_13: () = minecraft_spaghetti_roughness_modulator_19_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_14: () = minecraft_spaghetti_roughness_20_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_15: () = minecraft_pillar_32_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_16: () = minecraft_pillar_rareness_33_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_17: () = minecraft_pillar_thickness_34_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_18: () = minecraft_spaghetti_3d_rarity_22_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_19: () = minecraft_offset_3_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_0: () = density_function_RangeChoice_35_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_1: () = density_function_RangeChoice_37_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_2: () = density_function_RangeChoice_39_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_3: () = density_function_RangeChoice_41_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_4: () = density_function_Add_27_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_5: () = density_function_Multiply_18_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_6: () = density_function_Multiply_31_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_7: () = density_function_Noise_21_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_8: () = density_function_ShiftedNoise_2_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_9: () = density_function_ShiftedNoise_6_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_10: () = density_function_ShiftedNoise_9_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_0: () = density_function_Min_16_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_1: () = minecraft_overworld_ridges_folded_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_2: () = minecraft_overworld_erosion_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_3: () = minecraft_overworld_continents_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_4: () = minecraft_overworld_ridges_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_3_0: () = density_function_Cache2d_1_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_3_1: () = density_function_Cache2d_12_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_3_2: () = density_function_Cache2d_15_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_4_0: () = density_function_Add_0_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_5_0: () = final_density_d16x384x16_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            return (&self.buffers.final_density_d16x384x16_output);
        }
        
        pub fn orchestrate_vein_ridged(&mut self, origin: Vec3, host_consts: &HostConsts) -> &[f64; 98304] {
            let _kernel_call_0_0: () = minecraft_ore_vein_a_49_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_1: () = minecraft_ore_vein_b_51_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_0: () = density_function_RangeChoice_48_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_1: () = density_function_RangeChoice_50_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_0: () = vein_ridged_d16x384x16_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            return (&self.buffers.vein_ridged_d16x384x16_output);
        }
        
        pub fn orchestrate_vein_toggle(&mut self, origin: Vec3, host_consts: &HostConsts) -> &[f64; 98304] {
            let _kernel_call_0_0: () = minecraft_ore_veininess_53_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_0: () = density_function_RangeChoice_52_d5x49x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_0: () = vein_toggle_d16x384x16_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            return (&self.buffers.vein_toggle_d16x384x16_output);
        }
        
        pub fn orchestrate_preliminary_surface_level(&mut self, origin: Vec3, host_consts: &HostConsts) -> &[f64; 16384] {
            let _kernel_call_0_0: () = minecraft_offset_3_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_0: () = density_function_ShiftedNoise_55_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_1: () = density_function_ShiftedNoise_58_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_2: () = density_function_ShiftedNoise_61_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_0: () = minecraft_overworld_ridges_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_1: () = minecraft_overworld_ridges_folded_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_2: () = minecraft_overworld_erosion_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_3: () = minecraft_overworld_continents_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_3_0: () = density_function_Cache2d_64_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_3_1: () = density_function_Cache2d_54_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_4_0: () = density_function_66_d128x48x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_5_0: () = preliminary_surface_level_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            return (&self.buffers.preliminary_surface_level_d128x1x128_output);
        }
    }
    
}
