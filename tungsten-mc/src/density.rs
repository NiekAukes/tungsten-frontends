pub mod density_function {
    // Auto-generated inline Rust module from RCL
    #[allow(dead_code)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(warnings)]
    #[allow(clippy::all)]
    
    use super::*;
    use super::orchestration::*;
    
    const spline_coordinates_56: [f32; 17] = [0_f32, 0.05_f32, 0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32, 0.7_f32, 0.8_f32, 0.9_f32, 1_f32, 1.1_f32, 1.2_f32, 1.3_f32, 1.4_f32, 1.5_f32];
    const spline_derivatives_56: [f32; 17] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_57: [f32; 1] = [0_f32];
    const spline_coordinates_58: [f32; 1] = [0_f32];
    const spline_derivatives_57: [f32; 1] = [0.035_f32];
    const spline_derivatives_58: [f32; 1] = [0.045_f32];
    const spline_coordinates_59: [f32; 2] = [-0.3_f32, 0.3_f32];
    const spline_derivatives_59: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_60: [f32; 17] = [0_f32, 0.05_f32, 0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32, 0.7_f32, 0.8_f32, 0.9_f32, 1_f32, 1.1_f32, 1.2_f32, 1.3_f32, 1.4_f32, 1.5_f32];
    const spline_derivatives_60: [f32; 17] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_61: [f32; 1] = [0_f32];
    const spline_coordinates_62: [f32; 1] = [0_f32];
    const spline_derivatives_61: [f32; 1] = [0.04_f32];
    const spline_derivatives_62: [f32; 1] = [0.05_f32];
    const spline_coordinates_63: [f32; 2] = [-0.3_f32, 0.3_f32];
    const spline_derivatives_63: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_110: [f32; 20] = [0_f32, 0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32, 0.7_f32, 0.8_f32, 0.9_f32, 1_f32, 2_f32, 3_f32, 4_f32, 5_f32, 6_f32, 7_f32, 8_f32, 9_f32, 10_f32];
    const spline_derivatives_110: [f32; 20] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_109: [f32; 17] = [0_f32, 0.05_f32, 0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32, 0.7_f32, 0.8_f32, 0.9_f32, 1_f32, 1.1_f32, 1.2_f32, 1.3_f32, 1.4_f32, 1.5_f32];
    const spline_derivatives_109: [f32; 17] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_161: [f32; 17] = [0_f32, 0.05_f32, 0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32, 0.7_f32, 0.8_f32, 0.9_f32, 1_f32, 1.1_f32, 1.2_f32, 1.3_f32, 1.4_f32, 1.5_f32];
    const spline_derivatives_161: [f32; 17] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_162: [f32; 1] = [0_f32];
    const spline_coordinates_163: [f32; 1] = [0_f32];
    const spline_derivatives_162: [f32; 1] = [0.035_f32];
    const spline_derivatives_163: [f32; 1] = [0.045_f32];
    const spline_coordinates_164: [f32; 2] = [-0.3_f32, 0.3_f32];
    const spline_derivatives_164: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_165: [f32; 17] = [0_f32, 0.05_f32, 0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32, 0.7_f32, 0.8_f32, 0.9_f32, 1_f32, 1.1_f32, 1.2_f32, 1.3_f32, 1.4_f32, 1.5_f32];
    const spline_derivatives_165: [f32; 17] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_166: [f32; 1] = [0_f32];
    const spline_coordinates_167: [f32; 1] = [0_f32];
    const spline_derivatives_166: [f32; 1] = [0.04_f32];
    const spline_derivatives_167: [f32; 1] = [0.05_f32];
    const spline_coordinates_168: [f32; 2] = [-0.3_f32, 0.3_f32];
    const spline_derivatives_168: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_49: [f32; 20] = [0_f32, 0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32, 0.7_f32, 0.8_f32, 0.9_f32, 1_f32, 2_f32, 3_f32, 4_f32, 5_f32, 6_f32, 7_f32, 8_f32, 9_f32, 10_f32];
    const spline_derivatives_49: [f32; 20] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_48: [f32; 17] = [0_f32, 0.05_f32, 0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32, 0.7_f32, 0.8_f32, 0.9_f32, 1_f32, 1.1_f32, 1.2_f32, 1.3_f32, 1.4_f32, 1.5_f32];
    const spline_derivatives_48: [f32; 17] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_154: [f32; 20] = [0_f32, 0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32, 0.7_f32, 0.8_f32, 0.9_f32, 1_f32, 2_f32, 3_f32, 4_f32, 5_f32, 6_f32, 7_f32, 8_f32, 9_f32, 10_f32];
    const spline_derivatives_154: [f32; 20] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_153: [f32; 17] = [0_f32, 0.05_f32, 0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32, 0.7_f32, 0.8_f32, 0.9_f32, 1_f32, 1.1_f32, 1.2_f32, 1.3_f32, 1.4_f32, 1.5_f32];
    const spline_derivatives_153: [f32; 17] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_103: [f32; 1] = [0_f32];
    const spline_coordinates_104: [f32; 1] = [0_f32];
    const spline_derivatives_103: [f32; 1] = [1_f32];
    const spline_derivatives_104: [f32; 1] = [1_f32];
    const spline_coordinates_105: [f32; 2] = [-0.2_f32, 0.2_f32];
    const spline_derivatives_105: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_106: [f32; 1] = [0_f32];
    const spline_coordinates_107: [f32; 1] = [0_f32];
    const spline_derivatives_106: [f32; 1] = [0.03_f32];
    const spline_derivatives_107: [f32; 1] = [0.06_f32];
    const spline_coordinates_108: [f32; 2] = [-0.3_f32, 0.3_f32];
    const spline_derivatives_108: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_111: [f32; 1] = [0_f32];
    const spline_coordinates_112: [f32; 1] = [0_f32];
    const spline_derivatives_111: [f32; 1] = [1_f32];
    const spline_derivatives_112: [f32; 1] = [1_f32];
    const spline_coordinates_113: [f32; 2] = [-0.2_f32, 0.2_f32];
    const spline_derivatives_113: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_114: [f32; 1] = [0_f32];
    const spline_coordinates_115: [f32; 1] = [0_f32];
    const spline_derivatives_114: [f32; 1] = [0.03_f32];
    const spline_derivatives_115: [f32; 1] = [0.06_f32];
    const spline_coordinates_116: [f32; 2] = [-0.3_f32, 0.3_f32];
    const spline_derivatives_116: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_117: [f32; 17] = [0_f32, 0.05_f32, 0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32, 0.7_f32, 0.8_f32, 0.9_f32, 1_f32, 1.1_f32, 1.2_f32, 1.3_f32, 1.4_f32, 1.5_f32];
    const spline_derivatives_117: [f32; 17] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_118: [f32; 1] = [0_f32];
    const spline_coordinates_119: [f32; 1] = [0_f32];
    const spline_derivatives_118: [f32; 1] = [0.035_f32];
    const spline_derivatives_119: [f32; 1] = [0.045_f32];
    const spline_coordinates_120: [f32; 2] = [-0.3_f32, 0.3_f32];
    const spline_derivatives_120: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_121: [f32; 17] = [0_f32, 0.05_f32, 0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32, 0.7_f32, 0.8_f32, 0.9_f32, 1_f32, 1.1_f32, 1.2_f32, 1.3_f32, 1.4_f32, 1.5_f32];
    const spline_derivatives_121: [f32; 17] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_122: [f32; 1] = [0_f32];
    const spline_coordinates_123: [f32; 1] = [0_f32];
    const spline_derivatives_122: [f32; 1] = [0.04_f32];
    const spline_derivatives_123: [f32; 1] = [0.05_f32];
    const spline_coordinates_124: [f32; 2] = [-0.3_f32, 0.3_f32];
    const spline_derivatives_124: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_91: [f32; 1] = [0_f32];
    const spline_coordinates_92: [f32; 1] = [0_f32];
    const spline_derivatives_91: [f32; 1] = [1_f32];
    const spline_derivatives_92: [f32; 1] = [1_f32];
    const spline_coordinates_93: [f32; 2] = [0.5_f32, 0.6_f32];
    const spline_coordinates_94: [f32; 1] = [0_f32];
    const spline_derivatives_94: [f32; 1] = [1_f32];
    const spline_coordinates_95: [f32; 3] = [0.4_f32, 0.5_f32, 0.6_f32];
    const spline_coordinates_96: [f32; 1] = [0_f32];
    const spline_derivatives_96: [f32; 1] = [1_f32];
    const spline_coordinates_97: [f32; 4] = [0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const spline_coordinates_98: [f32; 1] = [0_f32];
    const spline_derivatives_98: [f32; 1] = [1_f32];
    const spline_coordinates_99: [f32; 5] = [0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const spline_coordinates_100: [f32; 1] = [0_f32];
    const spline_derivatives_100: [f32; 1] = [1_f32];
    const spline_coordinates_101: [f32; 6] = [0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const spline_derivatives_93: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_95: [f32; 3] = [0_f32, 0_f32, 0_f32];
    const spline_derivatives_97: [f32; 4] = [0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_99: [f32; 5] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_101: [f32; 6] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_102: [f32; 6] = [0.05_f32, 0.1_f32, 0.15_f32, 0.2_f32, 0.25_f32, 0.3_f32];
    const spline_derivatives_102: [f32; 6] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_42: [f32; 1] = [0_f32];
    const spline_coordinates_43: [f32; 1] = [0_f32];
    const spline_derivatives_42: [f32; 1] = [1_f32];
    const spline_derivatives_43: [f32; 1] = [1_f32];
    const spline_coordinates_44: [f32; 2] = [-0.2_f32, 0.2_f32];
    const spline_derivatives_44: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_45: [f32; 1] = [0_f32];
    const spline_coordinates_46: [f32; 1] = [0_f32];
    const spline_derivatives_45: [f32; 1] = [0.03_f32];
    const spline_derivatives_46: [f32; 1] = [0.06_f32];
    const spline_coordinates_47: [f32; 2] = [-0.3_f32, 0.3_f32];
    const spline_derivatives_47: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_50: [f32; 1] = [0_f32];
    const spline_coordinates_51: [f32; 1] = [0_f32];
    const spline_derivatives_50: [f32; 1] = [1_f32];
    const spline_derivatives_51: [f32; 1] = [1_f32];
    const spline_coordinates_52: [f32; 2] = [-0.2_f32, 0.2_f32];
    const spline_derivatives_52: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_53: [f32; 1] = [0_f32];
    const spline_coordinates_54: [f32; 1] = [0_f32];
    const spline_derivatives_53: [f32; 1] = [0.03_f32];
    const spline_derivatives_54: [f32; 1] = [0.06_f32];
    const spline_coordinates_55: [f32; 2] = [-0.3_f32, 0.3_f32];
    const spline_derivatives_55: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_147: [f32; 1] = [0_f32];
    const spline_coordinates_148: [f32; 1] = [0_f32];
    const spline_derivatives_147: [f32; 1] = [1_f32];
    const spline_derivatives_148: [f32; 1] = [1_f32];
    const spline_coordinates_149: [f32; 2] = [-0.2_f32, 0.2_f32];
    const spline_derivatives_149: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_150: [f32; 1] = [0_f32];
    const spline_coordinates_151: [f32; 1] = [0_f32];
    const spline_derivatives_150: [f32; 1] = [0.03_f32];
    const spline_derivatives_151: [f32; 1] = [0.06_f32];
    const spline_coordinates_152: [f32; 2] = [-0.3_f32, 0.3_f32];
    const spline_derivatives_152: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_155: [f32; 1] = [0_f32];
    const spline_coordinates_156: [f32; 1] = [0_f32];
    const spline_derivatives_155: [f32; 1] = [1_f32];
    const spline_derivatives_156: [f32; 1] = [1_f32];
    const spline_coordinates_157: [f32; 2] = [-0.2_f32, 0.2_f32];
    const spline_derivatives_157: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_158: [f32; 1] = [0_f32];
    const spline_coordinates_159: [f32; 1] = [0_f32];
    const spline_derivatives_158: [f32; 1] = [0.03_f32];
    const spline_derivatives_159: [f32; 1] = [0.06_f32];
    const spline_coordinates_160: [f32; 2] = [-0.3_f32, 0.3_f32];
    const spline_derivatives_160: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_29: [f32; 1] = [0_f32];
    const spline_derivatives_29: [f32; 1] = [1_f32];
    const spline_coordinates_30: [f32; 2] = [0.5_f32, 0.6_f32];
    const spline_coordinates_31: [f32; 1] = [0_f32];
    const spline_derivatives_31: [f32; 1] = [1_f32];
    const spline_coordinates_32: [f32; 2] = [0.5_f32, 0.6_f32];
    const spline_coordinates_33: [f32; 1] = [0_f32];
    const spline_derivatives_33: [f32; 1] = [1_f32];
    const spline_coordinates_34: [f32; 3] = [0.4_f32, 0.5_f32, 0.6_f32];
    const spline_coordinates_35: [f32; 1] = [0_f32];
    const spline_derivatives_35: [f32; 1] = [1_f32];
    const spline_coordinates_36: [f32; 4] = [0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const spline_coordinates_37: [f32; 1] = [0_f32];
    const spline_derivatives_37: [f32; 1] = [1_f32];
    const spline_coordinates_38: [f32; 5] = [0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const spline_coordinates_39: [f32; 1] = [0_f32];
    const spline_derivatives_39: [f32; 1] = [1_f32];
    const spline_coordinates_40: [f32; 6] = [0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const spline_derivatives_30: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_32: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_34: [f32; 3] = [0_f32, 0_f32, 0_f32];
    const spline_derivatives_36: [f32; 4] = [0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_38: [f32; 5] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_40: [f32; 6] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_41: [f32; 7] = [0_f32, 0.05_f32, 0.1_f32, 0.15_f32, 0.2_f32, 0.25_f32, 0.3_f32];
    const spline_derivatives_41: [f32; 7] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_134: [f32; 1] = [0_f32];
    const spline_derivatives_134: [f32; 1] = [1_f32];
    const spline_coordinates_135: [f32; 2] = [0.5_f32, 0.6_f32];
    const spline_coordinates_136: [f32; 1] = [0_f32];
    const spline_derivatives_136: [f32; 1] = [1_f32];
    const spline_coordinates_137: [f32; 2] = [0.5_f32, 0.6_f32];
    const spline_coordinates_138: [f32; 1] = [0_f32];
    const spline_derivatives_138: [f32; 1] = [1_f32];
    const spline_coordinates_139: [f32; 3] = [0.4_f32, 0.5_f32, 0.6_f32];
    const spline_coordinates_140: [f32; 1] = [0_f32];
    const spline_derivatives_140: [f32; 1] = [1_f32];
    const spline_coordinates_141: [f32; 4] = [0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const spline_coordinates_142: [f32; 1] = [0_f32];
    const spline_derivatives_142: [f32; 1] = [1_f32];
    const spline_coordinates_143: [f32; 5] = [0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const spline_coordinates_144: [f32; 1] = [0_f32];
    const spline_derivatives_144: [f32; 1] = [1_f32];
    const spline_coordinates_145: [f32; 6] = [0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const spline_derivatives_135: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_137: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_139: [f32; 3] = [0_f32, 0_f32, 0_f32];
    const spline_derivatives_141: [f32; 4] = [0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_143: [f32; 5] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_145: [f32; 6] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_146: [f32; 7] = [0_f32, 0.05_f32, 0.1_f32, 0.15_f32, 0.2_f32, 0.25_f32, 0.3_f32];
    const spline_derivatives_146: [f32; 7] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_169: [f32; 1] = [0_f32];
    const spline_derivatives_169: [f32; 1] = [1_f32];
    const spline_coordinates_170: [f32; 2] = [0.5_f32, 0.6_f32];
    const spline_coordinates_171: [f32; 1] = [0_f32];
    const spline_derivatives_171: [f32; 1] = [1_f32];
    const spline_coordinates_172: [f32; 2] = [0.5_f32, 0.6_f32];
    const spline_coordinates_173: [f32; 1] = [0_f32];
    const spline_derivatives_173: [f32; 1] = [1_f32];
    const spline_coordinates_174: [f32; 3] = [0.4_f32, 0.5_f32, 0.6_f32];
    const spline_coordinates_175: [f32; 1] = [0_f32];
    const spline_derivatives_175: [f32; 1] = [1_f32];
    const spline_coordinates_176: [f32; 4] = [0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const spline_coordinates_177: [f32; 1] = [0_f32];
    const spline_derivatives_177: [f32; 1] = [1_f32];
    const spline_coordinates_178: [f32; 5] = [0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const spline_coordinates_179: [f32; 1] = [0_f32];
    const spline_derivatives_179: [f32; 1] = [1_f32];
    const spline_coordinates_180: [f32; 6] = [0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const spline_derivatives_170: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_172: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_174: [f32; 3] = [0_f32, 0_f32, 0_f32];
    const spline_derivatives_176: [f32; 4] = [0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_178: [f32; 5] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_180: [f32; 6] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_181: [f32; 7] = [0_f32, 0.05_f32, 0.1_f32, 0.15_f32, 0.2_f32, 0.25_f32, 0.3_f32];
    const spline_derivatives_181: [f32; 7] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_182: [f32; 1] = [0_f32];
    const spline_derivatives_182: [f32; 1] = [1_f32];
    const spline_coordinates_183: [f32; 2] = [0.5_f32, 0.6_f32];
    const spline_coordinates_184: [f32; 1] = [0_f32];
    const spline_derivatives_184: [f32; 1] = [1_f32];
    const spline_coordinates_185: [f32; 2] = [0.5_f32, 0.6_f32];
    const spline_coordinates_186: [f32; 1] = [0_f32];
    const spline_derivatives_186: [f32; 1] = [1_f32];
    const spline_coordinates_187: [f32; 3] = [0.4_f32, 0.5_f32, 0.6_f32];
    const spline_coordinates_188: [f32; 1] = [0_f32];
    const spline_derivatives_188: [f32; 1] = [1_f32];
    const spline_coordinates_189: [f32; 4] = [0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const spline_coordinates_190: [f32; 1] = [0_f32];
    const spline_derivatives_190: [f32; 1] = [1_f32];
    const spline_coordinates_191: [f32; 5] = [0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const spline_coordinates_192: [f32; 1] = [0_f32];
    const spline_derivatives_192: [f32; 1] = [1_f32];
    const spline_coordinates_193: [f32; 6] = [0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const spline_derivatives_183: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_185: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_187: [f32; 3] = [0_f32, 0_f32, 0_f32];
    const spline_derivatives_189: [f32; 4] = [0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_191: [f32; 5] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_193: [f32; 6] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_194: [f32; 7] = [0_f32, 0.05_f32, 0.1_f32, 0.15_f32, 0.2_f32, 0.25_f32, 0.3_f32];
    const spline_derivatives_194: [f32; 7] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_195: [f32; 2] = [0.8_f32, 2_f32];
    const spline_derivatives_195: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_131: [f32; 1] = [0_f32];
    const spline_coordinates_132: [f32; 1] = [0_f32];
    const spline_derivatives_131: [f32; 1] = [1_f32];
    const spline_derivatives_132: [f32; 1] = [1_f32];
    const spline_coordinates_133: [f32; 2] = [0.6_f32, 0.8_f32];
    const spline_derivatives_133: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_64: [f32; 1] = [0_f32];
    const spline_derivatives_64: [f32; 1] = [1_f32];
    const spline_coordinates_65: [f32; 2] = [0.5_f32, 0.6_f32];
    const spline_coordinates_66: [f32; 1] = [0_f32];
    const spline_derivatives_66: [f32; 1] = [1_f32];
    const spline_coordinates_67: [f32; 2] = [0.5_f32, 0.6_f32];
    const spline_coordinates_68: [f32; 1] = [0_f32];
    const spline_derivatives_68: [f32; 1] = [1_f32];
    const spline_coordinates_69: [f32; 3] = [0.4_f32, 0.5_f32, 0.6_f32];
    const spline_coordinates_70: [f32; 1] = [0_f32];
    const spline_derivatives_70: [f32; 1] = [1_f32];
    const spline_coordinates_71: [f32; 4] = [0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const spline_coordinates_72: [f32; 1] = [0_f32];
    const spline_derivatives_72: [f32; 1] = [1_f32];
    const spline_coordinates_73: [f32; 5] = [0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const spline_coordinates_74: [f32; 1] = [0_f32];
    const spline_derivatives_74: [f32; 1] = [1_f32];
    const spline_coordinates_75: [f32; 6] = [0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const spline_derivatives_65: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_67: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_69: [f32; 3] = [0_f32, 0_f32, 0_f32];
    const spline_derivatives_71: [f32; 4] = [0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_73: [f32; 5] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_75: [f32; 6] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_76: [f32; 7] = [0_f32, 0.05_f32, 0.1_f32, 0.15_f32, 0.2_f32, 0.25_f32, 0.3_f32];
    const spline_derivatives_76: [f32; 7] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_77: [f32; 1] = [0_f32];
    const spline_derivatives_77: [f32; 1] = [1_f32];
    const spline_coordinates_78: [f32; 2] = [0.5_f32, 0.6_f32];
    const spline_coordinates_79: [f32; 1] = [0_f32];
    const spline_derivatives_79: [f32; 1] = [1_f32];
    const spline_coordinates_80: [f32; 2] = [0.5_f32, 0.6_f32];
    const spline_coordinates_81: [f32; 1] = [0_f32];
    const spline_derivatives_81: [f32; 1] = [1_f32];
    const spline_coordinates_82: [f32; 3] = [0.4_f32, 0.5_f32, 0.6_f32];
    const spline_coordinates_83: [f32; 1] = [0_f32];
    const spline_derivatives_83: [f32; 1] = [1_f32];
    const spline_coordinates_84: [f32; 4] = [0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const spline_coordinates_85: [f32; 1] = [0_f32];
    const spline_derivatives_85: [f32; 1] = [1_f32];
    const spline_coordinates_86: [f32; 5] = [0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const spline_coordinates_87: [f32; 1] = [0_f32];
    const spline_derivatives_87: [f32; 1] = [1_f32];
    const spline_coordinates_88: [f32; 6] = [0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const spline_derivatives_78: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_80: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_82: [f32; 3] = [0_f32, 0_f32, 0_f32];
    const spline_derivatives_84: [f32; 4] = [0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_86: [f32; 5] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_88: [f32; 6] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_89: [f32; 7] = [0_f32, 0.05_f32, 0.1_f32, 0.15_f32, 0.2_f32, 0.25_f32, 0.3_f32];
    const spline_derivatives_89: [f32; 7] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_90: [f32; 2] = [0.8_f32, 2_f32];
    const spline_derivatives_90: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_26: [f32; 1] = [0_f32];
    const spline_coordinates_27: [f32; 1] = [0_f32];
    const spline_derivatives_26: [f32; 1] = [1_f32];
    const spline_derivatives_27: [f32; 1] = [1_f32];
    const spline_coordinates_28: [f32; 2] = [0.6_f32, 0.8_f32];
    const spline_derivatives_28: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_2: [f32; 1] = [0_f32];
    const spline_coordinates_3: [f32; 1] = [0_f32];
    const spline_derivatives_2: [f32; 1] = [1_f32];
    const spline_derivatives_3: [f32; 1] = [1_f32];
    const spline_coordinates_4: [f32; 2] = [0_f32, 2_f32];
    const spline_derivatives_4: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_5: [f32; 1] = [0_f32];
    const spline_coordinates_6: [f32; 1] = [0_f32];
    const spline_derivatives_6: [f32; 1] = [1_f32];
    const spline_coordinates_7: [f32; 2] = [0_f32, 0.5_f32];
    const spline_coordinates_8: [f32; 1] = [0_f32];
    const spline_derivatives_8: [f32; 1] = [1_f32];
    const spline_coordinates_9: [f32; 2] = [0_f32, 0.5_f32];
    const spline_coordinates_10: [f32; 1] = [0_f32];
    const spline_derivatives_10: [f32; 1] = [1_f32];
    const spline_coordinates_11: [f32; 2] = [0_f32, 0.5_f32];
    const spline_coordinates_12: [f32; 1] = [0_f32];
    const spline_derivatives_7: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_9: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_11: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_12: [f32; 1] = [1_f32];
    const spline_coordinates_13: [f32; 4] = [0.1_f32, 0.35_f32, 0.7_f32, 1_f32];
    const spline_coordinates_14: [f32; 1] = [0_f32];
    const spline_derivatives_14: [f32; 1] = [1_f32];
    const spline_coordinates_15: [f32; 2] = [0_f32, 0.5_f32];
    const spline_coordinates_16: [f32; 1] = [0_f32];
    const spline_derivatives_16: [f32; 1] = [1_f32];
    const spline_coordinates_17: [f32; 2] = [0_f32, 0.5_f32];
    const spline_coordinates_18: [f32; 1] = [0_f32];
    const spline_derivatives_18: [f32; 1] = [1_f32];
    const spline_coordinates_19: [f32; 2] = [0_f32, 0.5_f32];
    const spline_derivatives_15: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_17: [f32; 2] = [0_f32, 0_f32];
    const spline_derivatives_19: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_20: [f32; 4] = [0.1_f32, 0.35_f32, 0.7_f32, 1_f32];
    const spline_derivatives_13: [f32; 4] = [0_f32, 0_f32, 0_f32, 0_f32];
    const spline_derivatives_20: [f32; 4] = [0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_21: [f32; 2] = [-1_f32, 1_f32];
    const spline_derivatives_5: [f32; 1] = [1_f32];
    const spline_derivatives_21: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_22: [f32; 2] = [0.02_f32, 0.05_f32];
    const spline_derivatives_22: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_130: [f32; 41] = [-2_f32, -1.9_f32, -1.8_f32, -1.7_f32, -1.6_f32, -1.5_f32, -1.4_f32, -1.3_f32, -1.2_f32, -1.1_f32, -1_f32, -0.9_f32, -0.8_f32, -0.7_f32, -0.6_f32, -0.5_f32, -0.4_f32, -0.3_f32, -0.2_f32, -0.1_f32, 0_f32, 0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32, 0.7_f32, 0.8_f32, 0.9_f32, 1_f32, 1.1_f32, 1.2_f32, 1.3_f32, 1.4_f32, 1.5_f32, 1.6_f32, 1.7_f32, 1.8_f32, 1.9_f32, 2_f32];
    const spline_derivatives_130: [f32; 41] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_129: [f32; 81] = [-2_f32, -1.95_f32, -1.9_f32, -1.85_f32, -1.8_f32, -1.75_f32, -1.7_f32, -1.65_f32, -1.6_f32, -1.55_f32, -1.5_f32, -1.45_f32, -1.4_f32, -1.35_f32, -1.3_f32, -1.25_f32, -1.2_f32, -1.15_f32, -1.1_f32, -1.05_f32, -1_f32, -0.95_f32, -0.9_f32, -0.85_f32, -0.8_f32, -0.75_f32, -0.7_f32, -0.65_f32, -0.6_f32, -0.55_f32, -0.5_f32, -0.45_f32, -0.4_f32, -0.35_f32, -0.3_f32, -0.25_f32, -0.2_f32, -0.15_f32, -0.1_f32, -0.05_f32, 0_f32, 0.05_f32, 0.1_f32, 0.15_f32, 0.2_f32, 0.25_f32, 0.3_f32, 0.35_f32, 0.4_f32, 0.45_f32, 0.5_f32, 0.55_f32, 0.6_f32, 0.65_f32, 0.7_f32, 0.75_f32, 0.8_f32, 0.85_f32, 0.9_f32, 0.95_f32, 1_f32, 1.05_f32, 1.1_f32, 1.15_f32, 1.2_f32, 1.25_f32, 1.3_f32, 1.35_f32, 1.4_f32, 1.45_f32, 1.5_f32, 1.55_f32, 1.6_f32, 1.65_f32, 1.7_f32, 1.75_f32, 1.8_f32, 1.85_f32, 1.9_f32, 1.95_f32, 2_f32];
    const spline_derivatives_129: [f32; 81] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_128: [f32; 165] = [-2_f32, -1.975_f32, -1.95_f32, -1.925_f32, -1.9_f32, -1.875_f32, -1.85_f32, -1.825_f32, -1.8_f32, -1.775_f32, -1.75_f32, -1.725_f32, -1.7_f32, -1.675_f32, -1.65_f32, -1.625_f32, -1.6_f32, -1.575_f32, -1.55_f32, -1.525_f32, -1.5_f32, -1.475_f32, -1.45_f32, -1.425_f32, -1.4_f32, -1.375_f32, -1.35_f32, -1.325_f32, -1.3_f32, -1.275_f32, -1.25_f32, -1.225_f32, -1.2_f32, -1.175_f32, -1.15_f32, -1.125_f32, -1.1_f32, -1.075_f32, -1.05_f32, -1.025_f32, -1.1_f32, -1.075_f32, -1.05_f32, -1.025_f32, -1_f32, -0.975_f32, -0.95_f32, -0.925_f32, -0.9_f32, -0.875_f32, -0.85_f32, -0.825_f32, -0.8_f32, -0.775_f32, -0.75_f32, -0.725_f32, -0.7_f32, -0.675_f32, -0.65_f32, -0.625_f32, -0.6_f32, -0.575_f32, -0.55_f32, -0.525_f32, -0.5_f32, -0.475_f32, -0.45_f32, -0.425_f32, -0.4_f32, -0.375_f32, -0.35_f32, -0.325_f32, -0.3_f32, -0.275_f32, -0.25_f32, -0.225_f32, -0.2_f32, -0.175_f32, -0.15_f32, -0.125_f32, -0.1_f32, -0.075_f32, -0.05_f32, -0.025_f32, 0_f32, 0.025_f32, 0.05_f32, 0.075_f32, 0.1_f32, 0.125_f32, 0.15_f32, 0.175_f32, 0.2_f32, 0.225_f32, 0.25_f32, 0.275_f32, 0.3_f32, 0.325_f32, 0.35_f32, 0.375_f32, 0.4_f32, 0.425_f32, 0.45_f32, 0.475_f32, 0.5_f32, 0.525_f32, 0.55_f32, 0.575_f32, 0.6_f32, 0.625_f32, 0.65_f32, 0.675_f32, 0.7_f32, 0.725_f32, 0.75_f32, 0.775_f32, 0.8_f32, 0.825_f32, 0.85_f32, 0.875_f32, 0.9_f32, 0.925_f32, 0.95_f32, 0.975_f32, 1_f32, 1.025_f32, 1.05_f32, 1.075_f32, 1.1_f32, 1.125_f32, 1.15_f32, 1.175_f32, 1.2_f32, 1.225_f32, 1.25_f32, 1.275_f32, 1.3_f32, 1.325_f32, 1.35_f32, 1.375_f32, 1.4_f32, 1.425_f32, 1.45_f32, 1.475_f32, 1.5_f32, 1.525_f32, 1.55_f32, 1.575_f32, 1.6_f32, 1.625_f32, 1.65_f32, 1.675_f32, 1.7_f32, 1.725_f32, 1.75_f32, 1.775_f32, 1.8_f32, 1.825_f32, 1.85_f32, 1.875_f32, 1.9_f32, 1.925_f32, 1.95_f32, 1.975_f32, 2_f32];
    const spline_derivatives_128: [f32; 165] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const spline_coordinates_23: [f32; 1] = [0_f32];
    const spline_coordinates_24: [f32; 1] = [0_f32];
    const spline_derivatives_23: [f32; 1] = [1_f32];
    const spline_derivatives_24: [f32; 1] = [1_f32];
    const spline_coordinates_25: [f32; 2] = [0.7_f32, 1.5_f32];
    const spline_derivatives_25: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates_125: [f32; 1] = [0_f32];
    const spline_coordinates_126: [f32; 1] = [0_f32];
    const spline_derivatives_125: [f32; 1] = [1_f32];
    const spline_derivatives_126: [f32; 1] = [1_f32];
    const spline_coordinates_127: [f32; 2] = [0.1_f32, 1_f32];
    const spline_derivatives_127: [f32; 2] = [0_f32, 0_f32];
    const spline_coordinates: [f32; 1] = [0_f32];
    const spline_derivatives: [f32; 1] = [1_f32];
    const spline_coordinates_1: [f32; 2] = [-63_f32, -54_f32];
    const spline_derivatives_1: [f32; 2] = [0_f32, 0_f32];
    #[inline]
    fn minecraft_cave_layer_66_d5x263x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(8_f64, 8_f64, 8_f64)) + (pos3 * Vec3::new(32_f64, 64_f64, 32_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_cave_layer_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_cave_layer_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_cave_layer_68_d5x263x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(6_f64, 6_f64, 6_f64)) + (pos3 * Vec3::new(24_f64, 48_f64, 24_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_cave_layer_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_cave_layer_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_cave_layer_71_d5x263x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(10_f64, 4_f64, 10_f64)) + (pos3 * Vec3::new(40_f64, 32_f64, 40_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_cave_layer_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_cave_layer_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_cave_layer_69_d5x263x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(4_f64, 4_f64, 4_f64)) + (pos3 * Vec3::new(16_f64, 32_f64, 16_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_cave_layer_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_cave_layer_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_cave_layer_75_d5x263x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.5_f64, 0.5_f64, 0.5_f64)) + (pos3 * Vec3::new(2_f64, 4_f64, 2_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_cave_layer_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_cave_layer_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_jagged_72_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(1000_f64, 0_f64, 1000_f64)) + (pos3 * Vec3::new(4000_f64, 0_f64, 4000_f64)));
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
    fn minecraft_cave_layer_73_d5x263x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(5_f64, 2_f64, 5_f64)) + (pos3 * Vec3::new(20_f64, 16_f64, 20_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_cave_layer_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_cave_layer_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_jagged_9_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(500_f64, 0_f64, 500_f64)) + (pos3 * Vec3::new(2000_f64, 0_f64, 2000_f64)));
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
    fn minecraft_jagged_9_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(500_f64, 0_f64, 500_f64)) + (pos3 * Vec3::new(2000_f64, 0_f64, 2000_f64)));
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
    fn minecraft_cave_layer_74_d5x263x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(2.5_f64, 1_f64, 2.5_f64)) + (pos3 * Vec3::new(10_f64, 8_f64, 10_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_cave_layer_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_cave_layer_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_jagged_18_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(250_f64, 0_f64, 250_f64)) + (pos3 * Vec3::new(1000_f64, 0_f64, 1000_f64)));
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
    fn minecraft_jagged_18_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(250_f64, 0_f64, 250_f64)) + (pos3 * Vec3::new(1000_f64, 0_f64, 1000_f64)));
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
    fn minecraft_ore_vein_a_78_d5x263x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
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
    fn minecraft_ore_vein_b_80_d5x263x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
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
    fn minecraft_ore_veininess_82_d5x263x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
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
    fn minecraft_gravel_70_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.03_f64, 0_f64, 0.03_f64)) + (pos3 * Vec3::new(0.12_f64, 0_f64, 0.12_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_cave_layer_1_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.2_f64, 0_f64, 0.2_f64)) + (pos3 * Vec3::new(0.8_f64, 0_f64, 0.8_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_cave_layer_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_cave_layer_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_cave_layer_1_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.2_f64, 0_f64, 0.2_f64)) + (pos3 * Vec3::new(0.8_f64, 0_f64, 0.8_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_cave_layer_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_cave_layer_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_jagged_5_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(1_f64, 0_f64, 1_f64)) + (pos3 * Vec3::new(4_f64, 0_f64, 4_f64)));
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
    fn minecraft_jagged_5_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(1_f64, 0_f64, 1_f64)) + (pos3 * Vec3::new(4_f64, 0_f64, 4_f64)));
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
    fn minecraft_cave_layer_10_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.15_f64, 0_f64, 0.15_f64)) + (pos3 * Vec3::new(0.6_f64, 0_f64, 0.6_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_cave_layer_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_cave_layer_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_cave_layer_10_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.15_f64, 0_f64, 0.15_f64)) + (pos3 * Vec3::new(0.6_f64, 0_f64, 0.6_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_cave_layer_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_cave_layer_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_jagged_21_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(300_f64, 0_f64, 300_f64)) + (pos3 * Vec3::new(1200_f64, 0_f64, 1200_f64)));
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
    fn minecraft_jagged_21_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(300_f64, 0_f64, 300_f64)) + (pos3 * Vec3::new(1200_f64, 0_f64, 1200_f64)));
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
    fn minecraft_gravel_31_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.12_f64, 0_f64, 0.12_f64)) + (pos3 * Vec3::new(0.48_f64, 0_f64, 0.48_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_gravel_31_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.12_f64, 0_f64, 0.12_f64)) + (pos3 * Vec3::new(0.48_f64, 0_f64, 0.48_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_cave_layer_7_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.1_f64, 0_f64, 0.1_f64)) + (pos3 * Vec3::new(0.4_f64, 0_f64, 0.4_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_cave_layer_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_cave_layer_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_cave_layer_7_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.1_f64, 0_f64, 0.1_f64)) + (pos3 * Vec3::new(0.4_f64, 0_f64, 0.4_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_cave_layer_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_cave_layer_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_gravel_8_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.1_f64, 0_f64, 0.1_f64)) + (pos3 * Vec3::new(0.4_f64, 0_f64, 0.4_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_gravel_8_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.1_f64, 0_f64, 0.1_f64)) + (pos3 * Vec3::new(0.4_f64, 0_f64, 0.4_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_gravel_2_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.05_f64, 0_f64, 0.05_f64)) + (pos3 * Vec3::new(0.2_f64, 0_f64, 0.2_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_gravel_2_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.05_f64, 0_f64, 0.05_f64)) + (pos3 * Vec3::new(0.2_f64, 0_f64, 0.2_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_jagged_3_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(175_f64, 0_f64, 175_f64)) + (pos3 * Vec3::new(700_f64, 0_f64, 700_f64)));
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
    fn minecraft_jagged_3_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(175_f64, 0_f64, 175_f64)) + (pos3 * Vec3::new(700_f64, 0_f64, 700_f64)));
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
    fn minecraft_gravel_19_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.2_f64, 0_f64, 0.2_f64)) + (pos3 * Vec3::new(0.8_f64, 0_f64, 0.8_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_gravel_19_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.2_f64, 0_f64, 0.2_f64)) + (pos3 * Vec3::new(0.8_f64, 0_f64, 0.8_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_cave_layer_22_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.3_f64, 0_f64, 0.3_f64)) + (pos3 * Vec3::new(1.2_f64, 0_f64, 1.2_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_cave_layer_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_cave_layer_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_cave_layer_22_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.3_f64, 0_f64, 0.3_f64)) + (pos3 * Vec3::new(1.2_f64, 0_f64, 1.2_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_cave_layer_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_cave_layer_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_gravel_23_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.11_f64, 0_f64, 0.11_f64)) + (pos3 * Vec3::new(0.44_f64, 0_f64, 0.44_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_gravel_23_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.11_f64, 0_f64, 0.11_f64)) + (pos3 * Vec3::new(0.44_f64, 0_f64, 0.44_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_cave_layer_30_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.25_f64, 0_f64, 0.25_f64)) + (pos3 * Vec3::new(1_f64, 0_f64, 1_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_cave_layer_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_cave_layer_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_cave_layer_30_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.25_f64, 0_f64, 0.25_f64)) + (pos3 * Vec3::new(1_f64, 0_f64, 1_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_cave_layer_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_cave_layer_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_jagged_29_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(501_f64, 0_f64, 501_f64)) + (pos3 * Vec3::new(2004_f64, 0_f64, 2004_f64)));
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
    fn minecraft_jagged_29_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(501_f64, 0_f64, 501_f64)) + (pos3 * Vec3::new(2004_f64, 0_f64, 2004_f64)));
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
    fn minecraft_jagged_39_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(2_f64, 0_f64, 2_f64)) + (pos3 * Vec3::new(8_f64, 0_f64, 8_f64)));
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
    fn minecraft_jagged_39_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(2_f64, 0_f64, 2_f64)) + (pos3 * Vec3::new(8_f64, 0_f64, 8_f64)));
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
    fn minecraft_jagged_37_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(600_f64, 0_f64, 600_f64)) + (pos3 * Vec3::new(2400_f64, 0_f64, 2400_f64)));
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
    fn minecraft_jagged_37_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(600_f64, 0_f64, 600_f64)) + (pos3 * Vec3::new(2400_f64, 0_f64, 2400_f64)));
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
    fn minecraft_gravel_11_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.25_f64, 0_f64, 0.25_f64)) + (pos3 * Vec3::new(1_f64, 0_f64, 1_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_gravel_11_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.25_f64, 0_f64, 0.25_f64)) + (pos3 * Vec3::new(1_f64, 0_f64, 1_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn density_function_RangeChoice_77_d5x263x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
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
            var_2 = buffers.minecraft_ore_vein_a_78_d5x263x5_output[index(p) as usize];
        } else {
            var_2 = 0_f64;
        }
        return var_2;
    }
    
    #[inline]
    fn density_function_RangeChoice_79_d5x263x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
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
            var_2 = buffers.minecraft_ore_vein_b_80_d5x263x5_output[index(p) as usize];
        } else {
            var_2 = 0_f64;
        }
        return var_2;
    }
    
    #[inline]
    fn density_function_RangeChoice_81_d5x263x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
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
            var_2 = buffers.minecraft_ore_veininess_82_d5x263x5_output[index(p) as usize];
        } else {
            var_2 = 0_f64;
        }
        return var_2;
    }
    
    #[inline]
    fn minecraft_determiner_cave_1_d5x263x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = clamp(((rpos3.y - -64_f64) / 2096_f64), 0_f64, 1_f64);
        return (((var * -2_f64) + 1_f64) + buffers.minecraft_gravel_70_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize]);
    }
    
    #[inline]
    fn density_function_Add_0_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return (((buffers.minecraft_cave_layer_1_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.09_f64) + -0.915_f64) + (((buffers.minecraft_gravel_2_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.05_f64) + 0_f64) + ((buffers.minecraft_jagged_3_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.015_f64) + 0_f64)));
    }
    
    #[inline]
    fn density_function_Abs_4_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return abs((buffers.minecraft_jagged_5_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] + 0_f64));
    }
    
    #[inline]
    fn density_function_Add_6_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return (((buffers.minecraft_cave_layer_7_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.1_f64) + -0.8_f64) + (((buffers.minecraft_gravel_8_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0_f64) + 0_f64) + ((buffers.minecraft_jagged_9_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.015_f64) + 0_f64)));
    }
    
    #[inline]
    fn minecraft_realism_mountains_slope_height_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 17];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut spline_values_1: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 1];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 2];
        let mut coordinate_3: f32;
        let mut spline_index_3: i32;
        let mut spline_result_f32_1: f32;
        let mut spline_result_f64_1: f64;
        let mut inlined_density_26: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        inlined_density_26 = minecraft_realism_mountains_jagged_original(p, config, buffers, auto_constants, host_consts);
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [1_f32, 0.95238096_f32, 0.90909094_f32, 0.8333333_f32, 0.7692308_f32, 0.71428573_f32, 0.6666667_f32, 0.625_f32, 0.5882353_f32, 0.5555556_f32, 0.5263158_f32, 0.5_f32, 0.47619048_f32, 0.45454547_f32, 0.4347826_f32, 0.41666666_f32, 0.4_f32];
        coordinate = (density_function_Multiply_24(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index = binary_search(spline_coordinates_56, (coordinate as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_56, spline_values, spline_derivatives_56, (coordinate as f32), (spline_index as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        spline_values_1 = [0_f32];
        coordinate_1 = (inlined_density_26 as f32);
        spline_index_1 = binary_search(spline_coordinates_57, (coordinate_1 as f32));
        spline_values_2 = [0_f32];
        coordinate_2 = (inlined_density_26 as f32);
        spline_index_2 = binary_search(spline_coordinates_58, (coordinate_2 as f32));
        spline_values_3 = [advanced_hermite(spline_coordinates_57, spline_values_1, spline_derivatives_57, (coordinate_1 as f32), (spline_index_1 as i32)), advanced_hermite(spline_coordinates_58, spline_values_2, spline_derivatives_58, (coordinate_2 as f32), (spline_index_2 as i32))];
        coordinate_3 = (minecraft_realism_mountains_factor_original(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_3 = binary_search(spline_coordinates_59, (coordinate_3 as f32));
        spline_result_f32_1 = advanced_hermite(spline_coordinates_59, spline_values_3, spline_derivatives_59, (coordinate_3 as f32), (spline_index_3 as i32));
        spline_result_f64_1 = (spline_result_f32_1 as f64);
        return ((0_f64 + ((buffers.minecraft_cave_layer_22_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.2_f64) + 0_f64)) + (((buffers.minecraft_gravel_23_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * spline_result_f64) * 0.7_f64) + spline_result_f64_1));
    }
    
    #[inline]
    fn minecraft_realism_mountains_jagged_original(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_jagged_29_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
    }
    
    #[inline]
    fn density_function_Multiply_24(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut var_1: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = (density_function_ShiftedNoise_25(p, config, buffers, auto_constants, host_consts) + (buffers.minecraft_gravel_23_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1_f64));
        var_1 = (density_function_ShiftedNoise_27(p, config, buffers, auto_constants, host_consts) + (buffers.minecraft_gravel_23_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1_f64));
        return ((((var * var) + (var_1 * var_1)) * 5000_f64) * 0.2_f64);
    }
    
    #[inline]
    fn density_function_ShiftedNoise_27(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return minecraft_gravel_shifted_28(p, config, buffers, auto_constants, host_consts, ((rpos3 * Vec3::new(0.11_f64, 0_f64, 0.11_f64)) + Vec3::new(0_f64, 0_f64, 1_f64)));
    }
    
    #[inline]
    fn minecraft_gravel_shifted_28(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn density_function_ShiftedNoise_25(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return minecraft_gravel_shifted_26(p, config, buffers, auto_constants, host_consts, ((rpos3 * Vec3::new(0.11_f64, 0_f64, 0.11_f64)) + Vec3::new(1_f64, 0_f64, 0_f64)));
    }
    
    #[inline]
    fn minecraft_gravel_shifted_26(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_realism_mountains_factor_original(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_gravel_23_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
    }
    
    #[inline]
    fn minecraft_realism_extreme_mountains_slope_height_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 17];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut spline_values_1: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 1];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 2];
        let mut coordinate_3: f32;
        let mut spline_index_3: i32;
        let mut spline_result_f32_1: f32;
        let mut spline_result_f64_1: f64;
        let mut inlined_density_26: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        inlined_density_26 = minecraft_realism_extreme_mountains_jagged_original(p, config, buffers, auto_constants, host_consts);
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [1_f32, 0.95238096_f32, 0.90909094_f32, 0.8333333_f32, 0.7692308_f32, 0.71428573_f32, 0.6666667_f32, 0.625_f32, 0.5882353_f32, 0.5555556_f32, 0.5263158_f32, 0.5_f32, 0.47619048_f32, 0.45454547_f32, 0.4347826_f32, 0.41666666_f32, 0.4_f32];
        coordinate = (density_function_Multiply_32(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index = binary_search(spline_coordinates_60, (coordinate as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_60, spline_values, spline_derivatives_60, (coordinate as f32), (spline_index as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        spline_values_1 = [0_f32];
        coordinate_1 = (inlined_density_26 as f32);
        spline_index_1 = binary_search(spline_coordinates_61, (coordinate_1 as f32));
        spline_values_2 = [0_f32];
        coordinate_2 = (inlined_density_26 as f32);
        spline_index_2 = binary_search(spline_coordinates_62, (coordinate_2 as f32));
        spline_values_3 = [advanced_hermite(spline_coordinates_61, spline_values_1, spline_derivatives_61, (coordinate_1 as f32), (spline_index_1 as i32)), advanced_hermite(spline_coordinates_62, spline_values_2, spline_derivatives_62, (coordinate_2 as f32), (spline_index_2 as i32))];
        coordinate_3 = (minecraft_realism_extreme_mountains_factor_original(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_3 = binary_search(spline_coordinates_63, (coordinate_3 as f32));
        spline_result_f32_1 = advanced_hermite(spline_coordinates_63, spline_values_3, spline_derivatives_63, (coordinate_3 as f32), (spline_index_3 as i32));
        spline_result_f64_1 = (spline_result_f32_1 as f64);
        return ((0_f64 + ((buffers.minecraft_cave_layer_30_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.2_f64) + 0.4_f64)) + (((buffers.minecraft_gravel_31_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * spline_result_f64) * 0.9_f64) + spline_result_f64_1));
    }
    
    #[inline]
    fn minecraft_realism_extreme_mountains_jagged_original(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_jagged_37_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
    }
    
    #[inline]
    fn density_function_Multiply_32(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut var_1: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = (density_function_ShiftedNoise_33(p, config, buffers, auto_constants, host_consts) + (buffers.minecraft_gravel_31_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1_f64));
        var_1 = (density_function_ShiftedNoise_35(p, config, buffers, auto_constants, host_consts) + (buffers.minecraft_gravel_31_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1_f64));
        return ((((var * var) + (var_1 * var_1)) * 5000_f64) * 0.2_f64);
    }
    
    #[inline]
    fn density_function_ShiftedNoise_35(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return minecraft_gravel_shifted_36(p, config, buffers, auto_constants, host_consts, ((rpos3 * Vec3::new(0.12_f64, 0_f64, 0.12_f64)) + Vec3::new(0_f64, 0_f64, 1_f64)));
    }
    
    #[inline]
    fn minecraft_gravel_shifted_36(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn density_function_ShiftedNoise_33(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return minecraft_gravel_shifted_34(p, config, buffers, auto_constants, host_consts, ((rpos3 * Vec3::new(0.12_f64, 0_f64, 0.12_f64)) + Vec3::new(1_f64, 0_f64, 0_f64)));
    }
    
    #[inline]
    fn minecraft_gravel_shifted_34(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_realism_extreme_mountains_factor_original(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_gravel_31_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
    }
    
    #[inline]
    fn density_function_Abs_38_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return abs(buffers.minecraft_jagged_39_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize]);
    }
    
    #[inline]
    fn minecraft_realism_extreme_hills_factor_original_d5x263x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_gravel_11_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
    }
    
    #[inline]
    fn density_function_Multiply_48_d5x263x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 17];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [1_f32, 0.95238096_f32, 0.90909094_f32, 0.8333333_f32, 0.7692308_f32, 0.71428573_f32, 0.6666667_f32, 0.625_f32, 0.5882353_f32, 0.5555556_f32, 0.5263158_f32, 0.5_f32, 0.47619048_f32, 0.45454547_f32, 0.4347826_f32, 0.41666666_f32, 0.4_f32];
        coordinate = (density_function_Multiply_49(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index = binary_search(spline_coordinates_109, (coordinate as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_109, spline_values, spline_derivatives_109, (coordinate as f32), (spline_index as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        return (buffers.minecraft_gravel_11_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * (spline_result_f64 * 0.3_f64));
    }
    
    #[inline]
    fn density_function_Multiply_49(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 20];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [0_f32, 0.31622776_f32, 0.4472136_f32, 0.5477226_f32, 0.6324555_f32, 0.70710677_f32, 0.7745967_f32, 0.83666_f32, 0.8944272_f32, 0.9486833_f32, 1_f32, 1.4142135_f32, 1.7320508_f32, 2_f32, 2.236068_f32, 2.4494898_f32, 2.6457512_f32, 2.828427_f32, 3_f32, 3.1622777_f32];
        coordinate = (minecraft_realism_extreme_hills_factor_divergence_squared_1(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index = binary_search(spline_coordinates_110, (coordinate as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_110, spline_values, spline_derivatives_110, (coordinate as f32), (spline_index as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        return (spline_result_f64 * 0.8_f64);
    }
    
    #[inline]
    fn minecraft_realism_extreme_hills_factor_divergence_squared_1(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut var_1: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = (density_function_ShiftedNoise_50(p, config, buffers, auto_constants, host_consts) + (buffers.minecraft_gravel_11_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1_f64));
        var_1 = (density_function_ShiftedNoise_52(p, config, buffers, auto_constants, host_consts) + (buffers.minecraft_gravel_11_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1_f64));
        return (((var * var) + (var_1 * var_1)) * 10000_f64);
    }
    
    #[inline]
    fn density_function_ShiftedNoise_52(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return minecraft_gravel_shifted_53(p, config, buffers, auto_constants, host_consts, ((rpos3 * Vec3::new(0.25_f64, 0_f64, 0.25_f64)) + Vec3::new(0_f64, 0_f64, 1_f64)));
    }
    
    #[inline]
    fn minecraft_gravel_shifted_53(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn density_function_ShiftedNoise_50(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return minecraft_gravel_shifted_51(p, config, buffers, auto_constants, host_consts, ((rpos3 * Vec3::new(0.25_f64, 0_f64, 0.25_f64)) + Vec3::new(1_f64, 0_f64, 0_f64)));
    }
    
    #[inline]
    fn minecraft_gravel_shifted_51(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn density_function_Add_83_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return (((buffers.minecraft_cave_layer_1_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize] * 0.09_f64) + -0.915_f64) + (((buffers.minecraft_gravel_2_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize] * 0.05_f64) + 0_f64) + ((buffers.minecraft_jagged_3_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize] * 0.015_f64) + 0_f64)));
    }
    
    #[inline]
    fn density_function_Abs_84_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return abs((buffers.minecraft_jagged_5_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize] + 0_f64));
    }
    
    #[inline]
    fn density_function_Add_85_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return (((buffers.minecraft_cave_layer_7_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize] * 0.1_f64) + -0.8_f64) + (((buffers.minecraft_gravel_8_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize] * 0_f64) + 0_f64) + ((buffers.minecraft_jagged_9_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize] * 0.015_f64) + 0_f64)));
    }
    
    #[inline]
    fn minecraft_realism_mountains_slope_height_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 17];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut spline_values_1: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 1];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 2];
        let mut coordinate_3: f32;
        let mut spline_index_3: i32;
        let mut spline_result_f32_1: f32;
        let mut spline_result_f64_1: f64;
        let mut inlined_density_26: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        inlined_density_26 = minecraft_realism_mountains_jagged_original_2(p, config, buffers, auto_constants, host_consts);
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [1_f32, 0.95238096_f32, 0.90909094_f32, 0.8333333_f32, 0.7692308_f32, 0.71428573_f32, 0.6666667_f32, 0.625_f32, 0.5882353_f32, 0.5555556_f32, 0.5263158_f32, 0.5_f32, 0.47619048_f32, 0.45454547_f32, 0.4347826_f32, 0.41666666_f32, 0.4_f32];
        coordinate = (density_function_Multiply_93(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index = binary_search(spline_coordinates_161, (coordinate as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_161, spline_values, spline_derivatives_161, (coordinate as f32), (spline_index as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        spline_values_1 = [0_f32];
        coordinate_1 = (inlined_density_26 as f32);
        spline_index_1 = binary_search(spline_coordinates_162, (coordinate_1 as f32));
        spline_values_2 = [0_f32];
        coordinate_2 = (inlined_density_26 as f32);
        spline_index_2 = binary_search(spline_coordinates_163, (coordinate_2 as f32));
        spline_values_3 = [advanced_hermite(spline_coordinates_162, spline_values_1, spline_derivatives_162, (coordinate_1 as f32), (spline_index_1 as i32)), advanced_hermite(spline_coordinates_163, spline_values_2, spline_derivatives_163, (coordinate_2 as f32), (spline_index_2 as i32))];
        coordinate_3 = (minecraft_realism_mountains_factor_original_2(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_3 = binary_search(spline_coordinates_164, (coordinate_3 as f32));
        spline_result_f32_1 = advanced_hermite(spline_coordinates_164, spline_values_3, spline_derivatives_164, (coordinate_3 as f32), (spline_index_3 as i32));
        spline_result_f64_1 = (spline_result_f32_1 as f64);
        return ((0_f64 + ((buffers.minecraft_cave_layer_22_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize] * 0.2_f64) + 0_f64)) + (((buffers.minecraft_gravel_23_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize] * spline_result_f64) * 0.7_f64) + spline_result_f64_1));
    }
    
    #[inline]
    fn minecraft_realism_mountains_jagged_original_2(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_jagged_29_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize];
    }
    
    #[inline]
    fn density_function_Multiply_93(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut var_1: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = (density_function_ShiftedNoise_94(p, config, buffers, auto_constants, host_consts) + (buffers.minecraft_gravel_23_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize] * -1_f64));
        var_1 = (density_function_ShiftedNoise_96(p, config, buffers, auto_constants, host_consts) + (buffers.minecraft_gravel_23_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize] * -1_f64));
        return ((((var * var) + (var_1 * var_1)) * 5000_f64) * 0.2_f64);
    }
    
    #[inline]
    fn density_function_ShiftedNoise_96(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return minecraft_gravel_shifted_97(p, config, buffers, auto_constants, host_consts, ((rpos3 * Vec3::new(0.11_f64, 0_f64, 0.11_f64)) + Vec3::new(0_f64, 0_f64, 1_f64)));
    }
    
    #[inline]
    fn minecraft_gravel_shifted_97(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn density_function_ShiftedNoise_94(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return minecraft_gravel_shifted_95(p, config, buffers, auto_constants, host_consts, ((rpos3 * Vec3::new(0.11_f64, 0_f64, 0.11_f64)) + Vec3::new(1_f64, 0_f64, 0_f64)));
    }
    
    #[inline]
    fn minecraft_gravel_shifted_95(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_realism_mountains_factor_original_2(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_gravel_23_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize];
    }
    
    #[inline]
    fn minecraft_realism_extreme_mountains_slope_height_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 17];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut spline_values_1: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 1];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 2];
        let mut coordinate_3: f32;
        let mut spline_index_3: i32;
        let mut spline_result_f32_1: f32;
        let mut spline_result_f64_1: f64;
        let mut inlined_density_26: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        inlined_density_26 = minecraft_realism_extreme_mountains_jagged_original_2(p, config, buffers, auto_constants, host_consts);
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [1_f32, 0.95238096_f32, 0.90909094_f32, 0.8333333_f32, 0.7692308_f32, 0.71428573_f32, 0.6666667_f32, 0.625_f32, 0.5882353_f32, 0.5555556_f32, 0.5263158_f32, 0.5_f32, 0.47619048_f32, 0.45454547_f32, 0.4347826_f32, 0.41666666_f32, 0.4_f32];
        coordinate = (density_function_Multiply_98(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index = binary_search(spline_coordinates_165, (coordinate as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_165, spline_values, spline_derivatives_165, (coordinate as f32), (spline_index as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        spline_values_1 = [0_f32];
        coordinate_1 = (inlined_density_26 as f32);
        spline_index_1 = binary_search(spline_coordinates_166, (coordinate_1 as f32));
        spline_values_2 = [0_f32];
        coordinate_2 = (inlined_density_26 as f32);
        spline_index_2 = binary_search(spline_coordinates_167, (coordinate_2 as f32));
        spline_values_3 = [advanced_hermite(spline_coordinates_166, spline_values_1, spline_derivatives_166, (coordinate_1 as f32), (spline_index_1 as i32)), advanced_hermite(spline_coordinates_167, spline_values_2, spline_derivatives_167, (coordinate_2 as f32), (spline_index_2 as i32))];
        coordinate_3 = (minecraft_realism_extreme_mountains_factor_original_2(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_3 = binary_search(spline_coordinates_168, (coordinate_3 as f32));
        spline_result_f32_1 = advanced_hermite(spline_coordinates_168, spline_values_3, spline_derivatives_168, (coordinate_3 as f32), (spline_index_3 as i32));
        spline_result_f64_1 = (spline_result_f32_1 as f64);
        return ((0_f64 + ((buffers.minecraft_cave_layer_30_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize] * 0.2_f64) + 0.4_f64)) + (((buffers.minecraft_gravel_31_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize] * spline_result_f64) * 0.9_f64) + spline_result_f64_1));
    }
    
    #[inline]
    fn minecraft_realism_extreme_mountains_jagged_original_2(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_jagged_37_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize];
    }
    
    #[inline]
    fn density_function_Multiply_98(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut var_1: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = (density_function_ShiftedNoise_99(p, config, buffers, auto_constants, host_consts) + (buffers.minecraft_gravel_31_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize] * -1_f64));
        var_1 = (density_function_ShiftedNoise_101(p, config, buffers, auto_constants, host_consts) + (buffers.minecraft_gravel_31_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize] * -1_f64));
        return ((((var * var) + (var_1 * var_1)) * 5000_f64) * 0.2_f64);
    }
    
    #[inline]
    fn density_function_ShiftedNoise_101(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return minecraft_gravel_shifted_102(p, config, buffers, auto_constants, host_consts, ((rpos3 * Vec3::new(0.12_f64, 0_f64, 0.12_f64)) + Vec3::new(0_f64, 0_f64, 1_f64)));
    }
    
    #[inline]
    fn minecraft_gravel_shifted_102(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn density_function_ShiftedNoise_99(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return minecraft_gravel_shifted_100(p, config, buffers, auto_constants, host_consts, ((rpos3 * Vec3::new(0.12_f64, 0_f64, 0.12_f64)) + Vec3::new(1_f64, 0_f64, 0_f64)));
    }
    
    #[inline]
    fn minecraft_gravel_shifted_100(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_realism_extreme_mountains_factor_original_2(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_gravel_31_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize];
    }
    
    #[inline]
    fn density_function_Abs_103_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return abs(buffers.minecraft_jagged_39_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize]);
    }
    
    #[inline]
    fn minecraft_realism_extreme_hills_factor_original_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_gravel_11_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
    }
    
    #[inline]
    fn density_function_Multiply_12_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 17];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [1_f32, 0.95238096_f32, 0.90909094_f32, 0.8333333_f32, 0.7692308_f32, 0.71428573_f32, 0.6666667_f32, 0.625_f32, 0.5882353_f32, 0.5555556_f32, 0.5263158_f32, 0.5_f32, 0.47619048_f32, 0.45454547_f32, 0.4347826_f32, 0.41666666_f32, 0.4_f32];
        coordinate = (density_function_Multiply_13(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index = binary_search(spline_coordinates_48, (coordinate as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_48, spline_values, spline_derivatives_48, (coordinate as f32), (spline_index as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        return (buffers.minecraft_gravel_11_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * (spline_result_f64 * 0.3_f64));
    }
    
    #[inline]
    fn density_function_Multiply_13(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 20];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [0_f32, 0.31622776_f32, 0.4472136_f32, 0.5477226_f32, 0.6324555_f32, 0.70710677_f32, 0.7745967_f32, 0.83666_f32, 0.8944272_f32, 0.9486833_f32, 1_f32, 1.4142135_f32, 1.7320508_f32, 2_f32, 2.236068_f32, 2.4494898_f32, 2.6457512_f32, 2.828427_f32, 3_f32, 3.1622777_f32];
        coordinate = (minecraft_realism_extreme_hills_factor_divergence_squared(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index = binary_search(spline_coordinates_49, (coordinate as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_49, spline_values, spline_derivatives_49, (coordinate as f32), (spline_index as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        return (spline_result_f64 * 0.8_f64);
    }
    
    #[inline]
    fn minecraft_realism_extreme_hills_factor_divergence_squared(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut var_1: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = (density_function_ShiftedNoise_14(p, config, buffers, auto_constants, host_consts) + (buffers.minecraft_gravel_11_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1_f64));
        var_1 = (density_function_ShiftedNoise_16(p, config, buffers, auto_constants, host_consts) + (buffers.minecraft_gravel_11_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1_f64));
        return (((var * var) + (var_1 * var_1)) * 10000_f64);
    }
    
    #[inline]
    fn density_function_ShiftedNoise_16(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return minecraft_gravel_shifted_17(p, config, buffers, auto_constants, host_consts, ((rpos3 * Vec3::new(0.25_f64, 0_f64, 0.25_f64)) + Vec3::new(0_f64, 0_f64, 1_f64)));
    }
    
    #[inline]
    fn minecraft_gravel_shifted_17(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn density_function_ShiftedNoise_14(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return minecraft_gravel_shifted_15(p, config, buffers, auto_constants, host_consts, ((rpos3 * Vec3::new(0.25_f64, 0_f64, 0.25_f64)) + Vec3::new(1_f64, 0_f64, 0_f64)));
    }
    
    #[inline]
    fn minecraft_gravel_shifted_15(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_realism_extreme_hills_factor_original_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_gravel_11_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize];
    }
    
    #[inline]
    fn density_function_Multiply_86_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 17];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [1_f32, 0.95238096_f32, 0.90909094_f32, 0.8333333_f32, 0.7692308_f32, 0.71428573_f32, 0.6666667_f32, 0.625_f32, 0.5882353_f32, 0.5555556_f32, 0.5263158_f32, 0.5_f32, 0.47619048_f32, 0.45454547_f32, 0.4347826_f32, 0.41666666_f32, 0.4_f32];
        coordinate = (density_function_Multiply_87(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index = binary_search(spline_coordinates_153, (coordinate as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_153, spline_values, spline_derivatives_153, (coordinate as f32), (spline_index as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        return (buffers.minecraft_gravel_11_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize] * (spline_result_f64 * 0.3_f64));
    }
    
    #[inline]
    fn density_function_Multiply_87(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 20];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [0_f32, 0.31622776_f32, 0.4472136_f32, 0.5477226_f32, 0.6324555_f32, 0.70710677_f32, 0.7745967_f32, 0.83666_f32, 0.8944272_f32, 0.9486833_f32, 1_f32, 1.4142135_f32, 1.7320508_f32, 2_f32, 2.236068_f32, 2.4494898_f32, 2.6457512_f32, 2.828427_f32, 3_f32, 3.1622777_f32];
        coordinate = (minecraft_realism_extreme_hills_factor_divergence_squared_2(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index = binary_search(spline_coordinates_154, (coordinate as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_154, spline_values, spline_derivatives_154, (coordinate as f32), (spline_index as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        return (spline_result_f64 * 0.8_f64);
    }
    
    #[inline]
    fn minecraft_realism_extreme_hills_factor_divergence_squared_2(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut var_1: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = (density_function_ShiftedNoise_88(p, config, buffers, auto_constants, host_consts) + (buffers.minecraft_gravel_11_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize] * -1_f64));
        var_1 = (density_function_ShiftedNoise_90(p, config, buffers, auto_constants, host_consts) + (buffers.minecraft_gravel_11_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize] * -1_f64));
        return (((var * var) + (var_1 * var_1)) * 10000_f64);
    }
    
    #[inline]
    fn density_function_ShiftedNoise_90(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return minecraft_gravel_shifted_91(p, config, buffers, auto_constants, host_consts, ((rpos3 * Vec3::new(0.25_f64, 0_f64, 0.25_f64)) + Vec3::new(0_f64, 0_f64, 1_f64)));
    }
    
    #[inline]
    fn minecraft_gravel_shifted_91(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn density_function_ShiftedNoise_88(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return minecraft_gravel_shifted_89(p, config, buffers, auto_constants, host_consts, ((rpos3 * Vec3::new(0.25_f64, 0_f64, 0.25_f64)) + Vec3::new(1_f64, 0_f64, 0_f64)));
    }
    
    #[inline]
    fn minecraft_gravel_shifted_89(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn vein_ridged_d16x2096x16(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return (-0.07999999821186066_f64 + max(abs(interpolate(buffers.density_function_RangeChoice_77_d5x263x5_output[cornerx0y0z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.density_function_RangeChoice_77_d5x263x5_output[cornerx4y0z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.density_function_RangeChoice_77_d5x263x5_output[cornerx0y8z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.density_function_RangeChoice_77_d5x263x5_output[cornerx4y8z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.density_function_RangeChoice_77_d5x263x5_output[cornerx0y0z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.density_function_RangeChoice_77_d5x263x5_output[cornerx4y0z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.density_function_RangeChoice_77_d5x263x5_output[cornerx0y8z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.density_function_RangeChoice_77_d5x263x5_output[cornerx4y8z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], xfract4(pos3), yfract8(pos3), zfract4(pos3))), abs(interpolate(buffers.density_function_RangeChoice_79_d5x263x5_output[cornerx0y0z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.density_function_RangeChoice_79_d5x263x5_output[cornerx4y0z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.density_function_RangeChoice_79_d5x263x5_output[cornerx0y8z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.density_function_RangeChoice_79_d5x263x5_output[cornerx4y8z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.density_function_RangeChoice_79_d5x263x5_output[cornerx0y0z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.density_function_RangeChoice_79_d5x263x5_output[cornerx4y0z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.density_function_RangeChoice_79_d5x263x5_output[cornerx0y8z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.density_function_RangeChoice_79_d5x263x5_output[cornerx4y8z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], xfract4(pos3), yfract8(pos3), zfract4(pos3)))));
    }
    
    #[inline]
    fn vein_toggle_d16x2096x16(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return interpolate(buffers.density_function_RangeChoice_81_d5x263x5_output[cornerx0y0z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.density_function_RangeChoice_81_d5x263x5_output[cornerx4y0z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.density_function_RangeChoice_81_d5x263x5_output[cornerx0y8z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.density_function_RangeChoice_81_d5x263x5_output[cornerx4y8z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.density_function_RangeChoice_81_d5x263x5_output[cornerx0y0z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.density_function_RangeChoice_81_d5x263x5_output[cornerx4y0z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.density_function_RangeChoice_81_d5x263x5_output[cornerx0y8z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.density_function_RangeChoice_81_d5x263x5_output[cornerx4y8z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], xfract4(pos3), yfract8(pos3), zfract4(pos3));
    }
    
    #[inline]
    fn minecraft_new_surface_combined_unprocessed_base_d5x263x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut spline_values: [f32; 1];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_values_1: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 2];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 1];
        let mut coordinate_3: f32;
        let mut spline_index_3: i32;
        let mut spline_values_4: [f32; 3];
        let mut coordinate_4: f32;
        let mut spline_index_4: i32;
        let mut spline_values_5: [f32; 1];
        let mut coordinate_5: f32;
        let mut spline_index_5: i32;
        let mut spline_values_6: [f32; 4];
        let mut coordinate_6: f32;
        let mut spline_index_6: i32;
        let mut spline_values_7: [f32; 1];
        let mut coordinate_7: f32;
        let mut spline_index_7: i32;
        let mut spline_values_8: [f32; 5];
        let mut coordinate_8: f32;
        let mut spline_index_8: i32;
        let mut spline_values_9: [f32; 1];
        let mut coordinate_9: f32;
        let mut spline_index_9: i32;
        let mut spline_values_10: [f32; 6];
        let mut coordinate_10: f32;
        let mut spline_index_10: i32;
        let mut spline_values_11: [f32; 6];
        let mut coordinate_11: f32;
        let mut spline_index_11: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut inlined_density_52: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        inlined_density_52 = density_function_Abs_47(p, config, buffers, auto_constants, host_consts);
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = clamp(((rpos3.y - -64_f64) / 2096_f64), 0_f64, 1_f64);
        spline_values = [0_f32];
        coordinate = (density_function_Add_45(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index = binary_search(spline_coordinates_91, (coordinate as f32));
        spline_values_1 = [0_f32];
        coordinate_1 = (density_function_Add_46(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_1 = binary_search(spline_coordinates_92, (coordinate_1 as f32));
        spline_values_2 = [advanced_hermite(spline_coordinates_91, spline_values, spline_derivatives_91, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_92, spline_values_1, spline_derivatives_92, (coordinate_1 as f32), (spline_index_1 as i32))];
        coordinate_2 = (inlined_density_52 as f32);
        spline_index_2 = binary_search(spline_coordinates_93, (coordinate_2 as f32));
        spline_values_3 = [0_f32];
        coordinate_3 = (minecraft_realism_hills_slope_height(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_3 = binary_search(spline_coordinates_94, (coordinate_3 as f32));
        spline_values_4 = [advanced_hermite(spline_coordinates_94, spline_values_3, spline_derivatives_94, (coordinate_3 as f32), (spline_index_3 as i32)), advanced_hermite(spline_coordinates_91, spline_values, spline_derivatives_91, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_92, spline_values_1, spline_derivatives_92, (coordinate_1 as f32), (spline_index_1 as i32))];
        coordinate_4 = (inlined_density_52 as f32);
        spline_index_4 = binary_search(spline_coordinates_95, (coordinate_4 as f32));
        spline_values_5 = [0_f32];
        coordinate_5 = (minecraft_realism_extreme_hills_slope_height(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_5 = binary_search(spline_coordinates_96, (coordinate_5 as f32));
        spline_values_6 = [advanced_hermite(spline_coordinates_96, spline_values_5, spline_derivatives_96, (coordinate_5 as f32), (spline_index_5 as i32)), advanced_hermite(spline_coordinates_94, spline_values_3, spline_derivatives_94, (coordinate_3 as f32), (spline_index_3 as i32)), advanced_hermite(spline_coordinates_91, spline_values, spline_derivatives_91, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_92, spline_values_1, spline_derivatives_92, (coordinate_1 as f32), (spline_index_1 as i32))];
        coordinate_6 = (inlined_density_52 as f32);
        spline_index_6 = binary_search(spline_coordinates_97, (coordinate_6 as f32));
        spline_values_7 = [0_f32];
        coordinate_7 = (minecraft_realism_mountains_slope_height(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_7 = binary_search(spline_coordinates_98, (coordinate_7 as f32));
        spline_values_8 = [advanced_hermite(spline_coordinates_98, spline_values_7, spline_derivatives_98, (coordinate_7 as f32), (spline_index_7 as i32)), advanced_hermite(spline_coordinates_96, spline_values_5, spline_derivatives_96, (coordinate_5 as f32), (spline_index_5 as i32)), advanced_hermite(spline_coordinates_94, spline_values_3, spline_derivatives_94, (coordinate_3 as f32), (spline_index_3 as i32)), advanced_hermite(spline_coordinates_91, spline_values, spline_derivatives_91, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_92, spline_values_1, spline_derivatives_92, (coordinate_1 as f32), (spline_index_1 as i32))];
        coordinate_8 = (inlined_density_52 as f32);
        spline_index_8 = binary_search(spline_coordinates_99, (coordinate_8 as f32));
        spline_values_9 = [0_f32];
        coordinate_9 = (minecraft_realism_extreme_mountains_slope_height(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_9 = binary_search(spline_coordinates_100, (coordinate_9 as f32));
        spline_values_10 = [advanced_hermite(spline_coordinates_100, spline_values_9, spline_derivatives_100, (coordinate_9 as f32), (spline_index_9 as i32)), advanced_hermite(spline_coordinates_98, spline_values_7, spline_derivatives_98, (coordinate_7 as f32), (spline_index_7 as i32)), advanced_hermite(spline_coordinates_96, spline_values_5, spline_derivatives_96, (coordinate_5 as f32), (spline_index_5 as i32)), advanced_hermite(spline_coordinates_94, spline_values_3, spline_derivatives_94, (coordinate_3 as f32), (spline_index_3 as i32)), advanced_hermite(spline_coordinates_91, spline_values, spline_derivatives_91, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_92, spline_values_1, spline_derivatives_92, (coordinate_1 as f32), (spline_index_1 as i32))];
        coordinate_10 = (inlined_density_52 as f32);
        spline_index_10 = binary_search(spline_coordinates_101, (coordinate_10 as f32));
        spline_values_11 = [advanced_hermite(spline_coordinates_93, spline_values_2, spline_derivatives_93, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_95, spline_values_4, spline_derivatives_95, (coordinate_4 as f32), (spline_index_4 as i32)), advanced_hermite(spline_coordinates_95, spline_values_4, spline_derivatives_95, (coordinate_4 as f32), (spline_index_4 as i32)), advanced_hermite(spline_coordinates_97, spline_values_6, spline_derivatives_97, (coordinate_6 as f32), (spline_index_6 as i32)), advanced_hermite(spline_coordinates_99, spline_values_8, spline_derivatives_99, (coordinate_8 as f32), (spline_index_8 as i32)), advanced_hermite(spline_coordinates_101, spline_values_10, spline_derivatives_101, (coordinate_10 as f32), (spline_index_10 as i32))];
        coordinate_11 = (density_function_Abs_65(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_11 = binary_search(spline_coordinates_102, (coordinate_11 as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_102, spline_values_11, spline_derivatives_102, (coordinate_11 as f32), (spline_index_11 as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        return (((var * -2_f64) + 1_f64) + spline_result_f64);
    }
    
    #[inline]
    fn density_function_Abs_47(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return abs((buffers.minecraft_jagged_5_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] + 0_f64));
    }
    
    #[inline]
    fn density_function_Add_45(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return (((buffers.minecraft_cave_layer_7_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.1_f64) + -0.8_f64) + (((buffers.minecraft_gravel_8_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0_f64) + 0_f64) + ((buffers.minecraft_jagged_9_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.015_f64) + 0_f64)));
    }
    
    #[inline]
    fn density_function_Add_46(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return (((buffers.minecraft_cave_layer_1_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.09_f64) + -0.915_f64) + (((buffers.minecraft_gravel_2_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.05_f64) + 0_f64) + ((buffers.minecraft_jagged_3_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.015_f64) + 0_f64)));
    }
    
    #[inline]
    fn minecraft_realism_hills_slope_height(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 1];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_values_1: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 2];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut spline_values_3: [f32; 1];
        let mut coordinate_3: f32;
        let mut spline_index_3: i32;
        let mut spline_values_4: [f32; 1];
        let mut coordinate_4: f32;
        let mut spline_index_4: i32;
        let mut spline_values_5: [f32; 2];
        let mut coordinate_5: f32;
        let mut spline_index_5: i32;
        let mut spline_result_f32_1: f32;
        let mut spline_result_f64_1: f64;
        let mut inlined_density_32: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        inlined_density_32 = minecraft_realism_hills_jagged_original_1(p, config, buffers, auto_constants, host_consts);
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [0_f32];
        coordinate = (buffers.minecraft_realism_extreme_hills_factor_original_d5x263x5_output[index(p) as usize] as f32);
        spline_index = binary_search(spline_coordinates_103, (coordinate as f32));
        spline_values_1 = [0_f32];
        coordinate_1 = (buffers.density_function_Multiply_48_d5x263x5_output[index(p) as usize] as f32);
        spline_index_1 = binary_search(spline_coordinates_104, (coordinate_1 as f32));
        spline_values_2 = [advanced_hermite(spline_coordinates_103, spline_values, spline_derivatives_103, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_104, spline_values_1, spline_derivatives_104, (coordinate_1 as f32), (spline_index_1 as i32))];
        coordinate_2 = (buffers.minecraft_realism_extreme_hills_factor_original_d5x263x5_output[index(p) as usize] as f32);
        spline_index_2 = binary_search(spline_coordinates_105, (coordinate_2 as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_105, spline_values_2, spline_derivatives_105, (coordinate_2 as f32), (spline_index_2 as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        spline_values_3 = [0_f32];
        coordinate_3 = (inlined_density_32 as f32);
        spline_index_3 = binary_search(spline_coordinates_106, (coordinate_3 as f32));
        spline_values_4 = [0_f32];
        coordinate_4 = (inlined_density_32 as f32);
        spline_index_4 = binary_search(spline_coordinates_107, (coordinate_4 as f32));
        spline_values_5 = [advanced_hermite(spline_coordinates_106, spline_values_3, spline_derivatives_106, (coordinate_3 as f32), (spline_index_3 as i32)), advanced_hermite(spline_coordinates_107, spline_values_4, spline_derivatives_107, (coordinate_4 as f32), (spline_index_4 as i32))];
        coordinate_5 = (minecraft_realism_hills_factor_original_1(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_5 = binary_search(spline_coordinates_108, (coordinate_5 as f32));
        spline_result_f32_1 = advanced_hermite(spline_coordinates_108, spline_values_5, spline_derivatives_108, (coordinate_5 as f32), (spline_index_5 as i32));
        spline_result_f64_1 = (spline_result_f32_1 as f64);
        return ((0_f64 + ((buffers.minecraft_cave_layer_10_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.25_f64) + -0.5_f64)) + ((spline_result_f64 * 0.2_f64) + spline_result_f64_1));
    }
    
    #[inline]
    fn minecraft_realism_hills_factor_original_1(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_gravel_19_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
    }
    
    #[inline]
    fn minecraft_realism_hills_jagged_original_1(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_jagged_18_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
    }
    
    #[inline]
    fn minecraft_realism_extreme_hills_slope_height(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 1];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_values_1: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 2];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut spline_values_3: [f32; 1];
        let mut coordinate_3: f32;
        let mut spline_index_3: i32;
        let mut spline_values_4: [f32; 1];
        let mut coordinate_4: f32;
        let mut spline_index_4: i32;
        let mut spline_values_5: [f32; 2];
        let mut coordinate_5: f32;
        let mut spline_index_5: i32;
        let mut spline_result_f32_1: f32;
        let mut spline_result_f64_1: f64;
        let mut inlined_density_31: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        inlined_density_31 = minecraft_realism_extreme_hills_jagged_original_1(p, config, buffers, auto_constants, host_consts);
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [0_f32];
        coordinate = (buffers.minecraft_realism_extreme_hills_factor_original_d5x263x5_output[index(p) as usize] as f32);
        spline_index = binary_search(spline_coordinates_111, (coordinate as f32));
        spline_values_1 = [0_f32];
        coordinate_1 = (buffers.density_function_Multiply_48_d5x263x5_output[index(p) as usize] as f32);
        spline_index_1 = binary_search(spline_coordinates_112, (coordinate_1 as f32));
        spline_values_2 = [advanced_hermite(spline_coordinates_111, spline_values, spline_derivatives_111, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_112, spline_values_1, spline_derivatives_112, (coordinate_1 as f32), (spline_index_1 as i32))];
        coordinate_2 = (buffers.minecraft_realism_extreme_hills_factor_original_d5x263x5_output[index(p) as usize] as f32);
        spline_index_2 = binary_search(spline_coordinates_113, (coordinate_2 as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_113, spline_values_2, spline_derivatives_113, (coordinate_2 as f32), (spline_index_2 as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        spline_values_3 = [0_f32];
        coordinate_3 = (inlined_density_31 as f32);
        spline_index_3 = binary_search(spline_coordinates_114, (coordinate_3 as f32));
        spline_values_4 = [0_f32];
        coordinate_4 = (inlined_density_31 as f32);
        spline_index_4 = binary_search(spline_coordinates_115, (coordinate_4 as f32));
        spline_values_5 = [advanced_hermite(spline_coordinates_114, spline_values_3, spline_derivatives_114, (coordinate_3 as f32), (spline_index_3 as i32)), advanced_hermite(spline_coordinates_115, spline_values_4, spline_derivatives_115, (coordinate_4 as f32), (spline_index_4 as i32))];
        coordinate_5 = (buffers.minecraft_realism_extreme_hills_factor_original_d5x263x5_output[index(p) as usize] as f32);
        spline_index_5 = binary_search(spline_coordinates_116, (coordinate_5 as f32));
        spline_result_f32_1 = advanced_hermite(spline_coordinates_116, spline_values_5, spline_derivatives_116, (coordinate_5 as f32), (spline_index_5 as i32));
        spline_result_f64_1 = (spline_result_f32_1 as f64);
        return ((0_f64 + ((buffers.minecraft_cave_layer_1_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.4_f64) + -0.2_f64)) + ((spline_result_f64 * 0.3_f64) + spline_result_f64_1));
    }
    
    #[inline]
    fn minecraft_realism_extreme_hills_jagged_original_1(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_jagged_21_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
    }
    
    #[inline]
    fn minecraft_realism_mountains_slope_height(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 17];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut spline_values_1: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 1];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 2];
        let mut coordinate_3: f32;
        let mut spline_index_3: i32;
        let mut spline_result_f32_1: f32;
        let mut spline_result_f64_1: f64;
        let mut inlined_density_26: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        inlined_density_26 = minecraft_realism_mountains_jagged_original_1(p, config, buffers, auto_constants, host_consts);
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [1_f32, 0.95238096_f32, 0.90909094_f32, 0.8333333_f32, 0.7692308_f32, 0.71428573_f32, 0.6666667_f32, 0.625_f32, 0.5882353_f32, 0.5555556_f32, 0.5263158_f32, 0.5_f32, 0.47619048_f32, 0.45454547_f32, 0.4347826_f32, 0.41666666_f32, 0.4_f32];
        coordinate = (density_function_Multiply_55(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index = binary_search(spline_coordinates_117, (coordinate as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_117, spline_values, spline_derivatives_117, (coordinate as f32), (spline_index as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        spline_values_1 = [0_f32];
        coordinate_1 = (inlined_density_26 as f32);
        spline_index_1 = binary_search(spline_coordinates_118, (coordinate_1 as f32));
        spline_values_2 = [0_f32];
        coordinate_2 = (inlined_density_26 as f32);
        spline_index_2 = binary_search(spline_coordinates_119, (coordinate_2 as f32));
        spline_values_3 = [advanced_hermite(spline_coordinates_118, spline_values_1, spline_derivatives_118, (coordinate_1 as f32), (spline_index_1 as i32)), advanced_hermite(spline_coordinates_119, spline_values_2, spline_derivatives_119, (coordinate_2 as f32), (spline_index_2 as i32))];
        coordinate_3 = (minecraft_realism_mountains_factor_original_1(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_3 = binary_search(spline_coordinates_120, (coordinate_3 as f32));
        spline_result_f32_1 = advanced_hermite(spline_coordinates_120, spline_values_3, spline_derivatives_120, (coordinate_3 as f32), (spline_index_3 as i32));
        spline_result_f64_1 = (spline_result_f32_1 as f64);
        return ((0_f64 + ((buffers.minecraft_cave_layer_22_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.2_f64) + 0_f64)) + (((buffers.minecraft_gravel_23_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * spline_result_f64) * 0.7_f64) + spline_result_f64_1));
    }
    
    #[inline]
    fn minecraft_realism_mountains_factor_original_1(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_gravel_23_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
    }
    
    #[inline]
    fn density_function_Multiply_55(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut var_1: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = (density_function_ShiftedNoise_56(p, config, buffers, auto_constants, host_consts) + (buffers.minecraft_gravel_23_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1_f64));
        var_1 = (density_function_ShiftedNoise_58(p, config, buffers, auto_constants, host_consts) + (buffers.minecraft_gravel_23_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1_f64));
        return ((((var * var) + (var_1 * var_1)) * 5000_f64) * 0.2_f64);
    }
    
    #[inline]
    fn density_function_ShiftedNoise_58(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return minecraft_gravel_shifted_59(p, config, buffers, auto_constants, host_consts, ((rpos3 * Vec3::new(0.11_f64, 0_f64, 0.11_f64)) + Vec3::new(0_f64, 0_f64, 1_f64)));
    }
    
    #[inline]
    fn minecraft_gravel_shifted_59(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn density_function_ShiftedNoise_56(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return minecraft_gravel_shifted_57(p, config, buffers, auto_constants, host_consts, ((rpos3 * Vec3::new(0.11_f64, 0_f64, 0.11_f64)) + Vec3::new(1_f64, 0_f64, 0_f64)));
    }
    
    #[inline]
    fn minecraft_gravel_shifted_57(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_realism_mountains_jagged_original_1(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_jagged_29_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
    }
    
    #[inline]
    fn minecraft_realism_extreme_mountains_slope_height(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 17];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut spline_values_1: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 1];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 2];
        let mut coordinate_3: f32;
        let mut spline_index_3: i32;
        let mut spline_result_f32_1: f32;
        let mut spline_result_f64_1: f64;
        let mut inlined_density_26: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        inlined_density_26 = minecraft_realism_extreme_mountains_jagged_original_1(p, config, buffers, auto_constants, host_consts);
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [1_f32, 0.95238096_f32, 0.90909094_f32, 0.8333333_f32, 0.7692308_f32, 0.71428573_f32, 0.6666667_f32, 0.625_f32, 0.5882353_f32, 0.5555556_f32, 0.5263158_f32, 0.5_f32, 0.47619048_f32, 0.45454547_f32, 0.4347826_f32, 0.41666666_f32, 0.4_f32];
        coordinate = (density_function_Multiply_60(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index = binary_search(spline_coordinates_121, (coordinate as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_121, spline_values, spline_derivatives_121, (coordinate as f32), (spline_index as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        spline_values_1 = [0_f32];
        coordinate_1 = (inlined_density_26 as f32);
        spline_index_1 = binary_search(spline_coordinates_122, (coordinate_1 as f32));
        spline_values_2 = [0_f32];
        coordinate_2 = (inlined_density_26 as f32);
        spline_index_2 = binary_search(spline_coordinates_123, (coordinate_2 as f32));
        spline_values_3 = [advanced_hermite(spline_coordinates_122, spline_values_1, spline_derivatives_122, (coordinate_1 as f32), (spline_index_1 as i32)), advanced_hermite(spline_coordinates_123, spline_values_2, spline_derivatives_123, (coordinate_2 as f32), (spline_index_2 as i32))];
        coordinate_3 = (minecraft_realism_extreme_mountains_factor_original_1(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_3 = binary_search(spline_coordinates_124, (coordinate_3 as f32));
        spline_result_f32_1 = advanced_hermite(spline_coordinates_124, spline_values_3, spline_derivatives_124, (coordinate_3 as f32), (spline_index_3 as i32));
        spline_result_f64_1 = (spline_result_f32_1 as f64);
        return ((0_f64 + ((buffers.minecraft_cave_layer_30_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.2_f64) + 0.4_f64)) + (((buffers.minecraft_gravel_31_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * spline_result_f64) * 0.9_f64) + spline_result_f64_1));
    }
    
    #[inline]
    fn minecraft_realism_extreme_mountains_factor_original_1(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_gravel_31_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
    }
    
    #[inline]
    fn density_function_Multiply_60(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut var_1: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = (density_function_ShiftedNoise_61(p, config, buffers, auto_constants, host_consts) + (buffers.minecraft_gravel_31_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1_f64));
        var_1 = (density_function_ShiftedNoise_63(p, config, buffers, auto_constants, host_consts) + (buffers.minecraft_gravel_31_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1_f64));
        return ((((var * var) + (var_1 * var_1)) * 5000_f64) * 0.2_f64);
    }
    
    #[inline]
    fn density_function_ShiftedNoise_63(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return minecraft_gravel_shifted_64(p, config, buffers, auto_constants, host_consts, ((rpos3 * Vec3::new(0.12_f64, 0_f64, 0.12_f64)) + Vec3::new(0_f64, 0_f64, 1_f64)));
    }
    
    #[inline]
    fn minecraft_gravel_shifted_64(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn density_function_ShiftedNoise_61(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return minecraft_gravel_shifted_62(p, config, buffers, auto_constants, host_consts, ((rpos3 * Vec3::new(0.12_f64, 0_f64, 0.12_f64)) + Vec3::new(1_f64, 0_f64, 0_f64)));
    }
    
    #[inline]
    fn minecraft_gravel_shifted_62(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_gravel_0_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_gravel_1_octave__8)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_gravel_0_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_gravel_1_octave__7)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), (&auto_constants.minecraft_gravel_0_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), (&auto_constants.minecraft_gravel_1_octave__6)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_gravel_0_octave__5)) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_gravel_1_octave__5)) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_realism_extreme_mountains_jagged_original_1(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_jagged_37_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
    }
    
    #[inline]
    fn density_function_Abs_65(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return abs(buffers.minecraft_jagged_39_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize]);
    }
    
    #[inline]
    fn minecraft_realism_hills_slope_height_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 1];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_values_1: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 2];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut spline_values_3: [f32; 1];
        let mut coordinate_3: f32;
        let mut spline_index_3: i32;
        let mut spline_values_4: [f32; 1];
        let mut coordinate_4: f32;
        let mut spline_index_4: i32;
        let mut spline_values_5: [f32; 2];
        let mut coordinate_5: f32;
        let mut spline_index_5: i32;
        let mut spline_result_f32_1: f32;
        let mut spline_result_f64_1: f64;
        let mut inlined_density_32: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        inlined_density_32 = minecraft_realism_hills_jagged_original(p, config, buffers, auto_constants, host_consts);
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [0_f32];
        coordinate = (buffers.minecraft_realism_extreme_hills_factor_original_d5x1x5_output[index(p) as usize] as f32);
        spline_index = binary_search(spline_coordinates_42, (coordinate as f32));
        spline_values_1 = [0_f32];
        coordinate_1 = (buffers.density_function_Multiply_12_d5x1x5_output[index(p) as usize] as f32);
        spline_index_1 = binary_search(spline_coordinates_43, (coordinate_1 as f32));
        spline_values_2 = [advanced_hermite(spline_coordinates_42, spline_values, spline_derivatives_42, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_43, spline_values_1, spline_derivatives_43, (coordinate_1 as f32), (spline_index_1 as i32))];
        coordinate_2 = (buffers.minecraft_realism_extreme_hills_factor_original_d5x1x5_output[index(p) as usize] as f32);
        spline_index_2 = binary_search(spline_coordinates_44, (coordinate_2 as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_44, spline_values_2, spline_derivatives_44, (coordinate_2 as f32), (spline_index_2 as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        spline_values_3 = [0_f32];
        coordinate_3 = (inlined_density_32 as f32);
        spline_index_3 = binary_search(spline_coordinates_45, (coordinate_3 as f32));
        spline_values_4 = [0_f32];
        coordinate_4 = (inlined_density_32 as f32);
        spline_index_4 = binary_search(spline_coordinates_46, (coordinate_4 as f32));
        spline_values_5 = [advanced_hermite(spline_coordinates_45, spline_values_3, spline_derivatives_45, (coordinate_3 as f32), (spline_index_3 as i32)), advanced_hermite(spline_coordinates_46, spline_values_4, spline_derivatives_46, (coordinate_4 as f32), (spline_index_4 as i32))];
        coordinate_5 = (minecraft_realism_hills_factor_original(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_5 = binary_search(spline_coordinates_47, (coordinate_5 as f32));
        spline_result_f32_1 = advanced_hermite(spline_coordinates_47, spline_values_5, spline_derivatives_47, (coordinate_5 as f32), (spline_index_5 as i32));
        spline_result_f64_1 = (spline_result_f32_1 as f64);
        return ((0_f64 + ((buffers.minecraft_cave_layer_10_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.25_f64) + -0.5_f64)) + ((spline_result_f64 * 0.2_f64) + spline_result_f64_1));
    }
    
    #[inline]
    fn minecraft_realism_hills_jagged_original(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_jagged_18_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
    }
    
    #[inline]
    fn minecraft_realism_hills_factor_original(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_gravel_19_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
    }
    
    #[inline]
    fn minecraft_realism_extreme_hills_slope_height_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 1];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_values_1: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 2];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut spline_values_3: [f32; 1];
        let mut coordinate_3: f32;
        let mut spline_index_3: i32;
        let mut spline_values_4: [f32; 1];
        let mut coordinate_4: f32;
        let mut spline_index_4: i32;
        let mut spline_values_5: [f32; 2];
        let mut coordinate_5: f32;
        let mut spline_index_5: i32;
        let mut spline_result_f32_1: f32;
        let mut spline_result_f64_1: f64;
        let mut inlined_density_31: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        inlined_density_31 = minecraft_realism_extreme_hills_jagged_original(p, config, buffers, auto_constants, host_consts);
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [0_f32];
        coordinate = (buffers.minecraft_realism_extreme_hills_factor_original_d5x1x5_output[index(p) as usize] as f32);
        spline_index = binary_search(spline_coordinates_50, (coordinate as f32));
        spline_values_1 = [0_f32];
        coordinate_1 = (buffers.density_function_Multiply_12_d5x1x5_output[index(p) as usize] as f32);
        spline_index_1 = binary_search(spline_coordinates_51, (coordinate_1 as f32));
        spline_values_2 = [advanced_hermite(spline_coordinates_50, spline_values, spline_derivatives_50, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_51, spline_values_1, spline_derivatives_51, (coordinate_1 as f32), (spline_index_1 as i32))];
        coordinate_2 = (buffers.minecraft_realism_extreme_hills_factor_original_d5x1x5_output[index(p) as usize] as f32);
        spline_index_2 = binary_search(spline_coordinates_52, (coordinate_2 as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_52, spline_values_2, spline_derivatives_52, (coordinate_2 as f32), (spline_index_2 as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        spline_values_3 = [0_f32];
        coordinate_3 = (inlined_density_31 as f32);
        spline_index_3 = binary_search(spline_coordinates_53, (coordinate_3 as f32));
        spline_values_4 = [0_f32];
        coordinate_4 = (inlined_density_31 as f32);
        spline_index_4 = binary_search(spline_coordinates_54, (coordinate_4 as f32));
        spline_values_5 = [advanced_hermite(spline_coordinates_53, spline_values_3, spline_derivatives_53, (coordinate_3 as f32), (spline_index_3 as i32)), advanced_hermite(spline_coordinates_54, spline_values_4, spline_derivatives_54, (coordinate_4 as f32), (spline_index_4 as i32))];
        coordinate_5 = (buffers.minecraft_realism_extreme_hills_factor_original_d5x1x5_output[index(p) as usize] as f32);
        spline_index_5 = binary_search(spline_coordinates_55, (coordinate_5 as f32));
        spline_result_f32_1 = advanced_hermite(spline_coordinates_55, spline_values_5, spline_derivatives_55, (coordinate_5 as f32), (spline_index_5 as i32));
        spline_result_f64_1 = (spline_result_f32_1 as f64);
        return ((0_f64 + ((buffers.minecraft_cave_layer_1_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.4_f64) + -0.2_f64)) + ((spline_result_f64 * 0.3_f64) + spline_result_f64_1));
    }
    
    #[inline]
    fn minecraft_realism_extreme_hills_jagged_original(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_jagged_21_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
    }
    
    #[inline]
    fn minecraft_realism_hills_slope_height_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 1];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_values_1: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 2];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut spline_values_3: [f32; 1];
        let mut coordinate_3: f32;
        let mut spline_index_3: i32;
        let mut spline_values_4: [f32; 1];
        let mut coordinate_4: f32;
        let mut spline_index_4: i32;
        let mut spline_values_5: [f32; 2];
        let mut coordinate_5: f32;
        let mut spline_index_5: i32;
        let mut spline_result_f32_1: f32;
        let mut spline_result_f64_1: f64;
        let mut inlined_density_32: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        inlined_density_32 = minecraft_realism_hills_jagged_original_2(p, config, buffers, auto_constants, host_consts);
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [0_f32];
        coordinate = (buffers.minecraft_realism_extreme_hills_factor_original_d128x1x128_output[index(p) as usize] as f32);
        spline_index = binary_search(spline_coordinates_147, (coordinate as f32));
        spline_values_1 = [0_f32];
        coordinate_1 = (buffers.density_function_Multiply_86_d128x1x128_output[index(p) as usize] as f32);
        spline_index_1 = binary_search(spline_coordinates_148, (coordinate_1 as f32));
        spline_values_2 = [advanced_hermite(spline_coordinates_147, spline_values, spline_derivatives_147, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_148, spline_values_1, spline_derivatives_148, (coordinate_1 as f32), (spline_index_1 as i32))];
        coordinate_2 = (buffers.minecraft_realism_extreme_hills_factor_original_d128x1x128_output[index(p) as usize] as f32);
        spline_index_2 = binary_search(spline_coordinates_149, (coordinate_2 as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_149, spline_values_2, spline_derivatives_149, (coordinate_2 as f32), (spline_index_2 as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        spline_values_3 = [0_f32];
        coordinate_3 = (inlined_density_32 as f32);
        spline_index_3 = binary_search(spline_coordinates_150, (coordinate_3 as f32));
        spline_values_4 = [0_f32];
        coordinate_4 = (inlined_density_32 as f32);
        spline_index_4 = binary_search(spline_coordinates_151, (coordinate_4 as f32));
        spline_values_5 = [advanced_hermite(spline_coordinates_150, spline_values_3, spline_derivatives_150, (coordinate_3 as f32), (spline_index_3 as i32)), advanced_hermite(spline_coordinates_151, spline_values_4, spline_derivatives_151, (coordinate_4 as f32), (spline_index_4 as i32))];
        coordinate_5 = (minecraft_realism_hills_factor_original_2(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_5 = binary_search(spline_coordinates_152, (coordinate_5 as f32));
        spline_result_f32_1 = advanced_hermite(spline_coordinates_152, spline_values_5, spline_derivatives_152, (coordinate_5 as f32), (spline_index_5 as i32));
        spline_result_f64_1 = (spline_result_f32_1 as f64);
        return ((0_f64 + ((buffers.minecraft_cave_layer_10_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize] * 0.25_f64) + -0.5_f64)) + ((spline_result_f64 * 0.2_f64) + spline_result_f64_1));
    }
    
    #[inline]
    fn minecraft_realism_hills_jagged_original_2(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_jagged_18_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize];
    }
    
    #[inline]
    fn minecraft_realism_hills_factor_original_2(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_gravel_19_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize];
    }
    
    #[inline]
    fn minecraft_realism_extreme_hills_slope_height_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 1];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_values_1: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 2];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut spline_values_3: [f32; 1];
        let mut coordinate_3: f32;
        let mut spline_index_3: i32;
        let mut spline_values_4: [f32; 1];
        let mut coordinate_4: f32;
        let mut spline_index_4: i32;
        let mut spline_values_5: [f32; 2];
        let mut coordinate_5: f32;
        let mut spline_index_5: i32;
        let mut spline_result_f32_1: f32;
        let mut spline_result_f64_1: f64;
        let mut inlined_density_31: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        inlined_density_31 = minecraft_realism_extreme_hills_jagged_original_2(p, config, buffers, auto_constants, host_consts);
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [0_f32];
        coordinate = (buffers.minecraft_realism_extreme_hills_factor_original_d128x1x128_output[index(p) as usize] as f32);
        spline_index = binary_search(spline_coordinates_155, (coordinate as f32));
        spline_values_1 = [0_f32];
        coordinate_1 = (buffers.density_function_Multiply_86_d128x1x128_output[index(p) as usize] as f32);
        spline_index_1 = binary_search(spline_coordinates_156, (coordinate_1 as f32));
        spline_values_2 = [advanced_hermite(spline_coordinates_155, spline_values, spline_derivatives_155, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_156, spline_values_1, spline_derivatives_156, (coordinate_1 as f32), (spline_index_1 as i32))];
        coordinate_2 = (buffers.minecraft_realism_extreme_hills_factor_original_d128x1x128_output[index(p) as usize] as f32);
        spline_index_2 = binary_search(spline_coordinates_157, (coordinate_2 as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_157, spline_values_2, spline_derivatives_157, (coordinate_2 as f32), (spline_index_2 as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        spline_values_3 = [0_f32];
        coordinate_3 = (inlined_density_31 as f32);
        spline_index_3 = binary_search(spline_coordinates_158, (coordinate_3 as f32));
        spline_values_4 = [0_f32];
        coordinate_4 = (inlined_density_31 as f32);
        spline_index_4 = binary_search(spline_coordinates_159, (coordinate_4 as f32));
        spline_values_5 = [advanced_hermite(spline_coordinates_158, spline_values_3, spline_derivatives_158, (coordinate_3 as f32), (spline_index_3 as i32)), advanced_hermite(spline_coordinates_159, spline_values_4, spline_derivatives_159, (coordinate_4 as f32), (spline_index_4 as i32))];
        coordinate_5 = (buffers.minecraft_realism_extreme_hills_factor_original_d128x1x128_output[index(p) as usize] as f32);
        spline_index_5 = binary_search(spline_coordinates_160, (coordinate_5 as f32));
        spline_result_f32_1 = advanced_hermite(spline_coordinates_160, spline_values_5, spline_derivatives_160, (coordinate_5 as f32), (spline_index_5 as i32));
        spline_result_f64_1 = (spline_result_f32_1 as f64);
        return ((0_f64 + ((buffers.minecraft_cave_layer_1_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize] * 0.4_f64) + -0.2_f64)) + ((spline_result_f64 * 0.3_f64) + spline_result_f64_1));
    }
    
    #[inline]
    fn minecraft_realism_extreme_hills_jagged_original_2(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_jagged_21_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize];
    }
    
    #[inline]
    fn minecraft_new_surface_height_unprocessed_rivered_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 1];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_values_1: [f32; 2];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 1];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 2];
        let mut coordinate_3: f32;
        let mut spline_index_3: i32;
        let mut spline_values_4: [f32; 1];
        let mut coordinate_4: f32;
        let mut spline_index_4: i32;
        let mut spline_values_5: [f32; 3];
        let mut coordinate_5: f32;
        let mut spline_index_5: i32;
        let mut spline_values_6: [f32; 1];
        let mut coordinate_6: f32;
        let mut spline_index_6: i32;
        let mut spline_values_7: [f32; 4];
        let mut coordinate_7: f32;
        let mut spline_index_7: i32;
        let mut spline_values_8: [f32; 1];
        let mut coordinate_8: f32;
        let mut spline_index_8: i32;
        let mut spline_values_9: [f32; 5];
        let mut coordinate_9: f32;
        let mut spline_index_9: i32;
        let mut spline_values_10: [f32; 1];
        let mut coordinate_10: f32;
        let mut spline_index_10: i32;
        let mut spline_values_11: [f32; 6];
        let mut coordinate_11: f32;
        let mut spline_index_11: i32;
        let mut spline_values_12: [f32; 7];
        let mut coordinate_12: f32;
        let mut spline_index_12: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [0_f32];
        coordinate = (buffers.density_function_Add_0_d5x1x5_output[index(p) as usize] as f32);
        spline_index = binary_search(spline_coordinates_29, (coordinate as f32));
        spline_values_1 = [-0.9_f32, advanced_hermite(spline_coordinates_29, spline_values, spline_derivatives_29, (coordinate as f32), (spline_index as i32))];
        coordinate_1 = (buffers.density_function_Abs_4_d5x1x5_output[index(p) as usize] as f32);
        spline_index_1 = binary_search(spline_coordinates_30, (coordinate_1 as f32));
        spline_values_2 = [0_f32];
        coordinate_2 = (buffers.density_function_Add_6_d5x1x5_output[index(p) as usize] as f32);
        spline_index_2 = binary_search(spline_coordinates_31, (coordinate_2 as f32));
        spline_values_3 = [advanced_hermite(spline_coordinates_31, spline_values_2, spline_derivatives_31, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_29, spline_values, spline_derivatives_29, (coordinate as f32), (spline_index as i32))];
        coordinate_3 = (buffers.density_function_Abs_4_d5x1x5_output[index(p) as usize] as f32);
        spline_index_3 = binary_search(spline_coordinates_32, (coordinate_3 as f32));
        spline_values_4 = [0_f32];
        coordinate_4 = (buffers.minecraft_realism_hills_slope_height_d5x1x5_output[index(p) as usize] as f32);
        spline_index_4 = binary_search(spline_coordinates_33, (coordinate_4 as f32));
        spline_values_5 = [advanced_hermite(spline_coordinates_33, spline_values_4, spline_derivatives_33, (coordinate_4 as f32), (spline_index_4 as i32)), advanced_hermite(spline_coordinates_31, spline_values_2, spline_derivatives_31, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_29, spline_values, spline_derivatives_29, (coordinate as f32), (spline_index as i32))];
        coordinate_5 = (buffers.density_function_Abs_4_d5x1x5_output[index(p) as usize] as f32);
        spline_index_5 = binary_search(spline_coordinates_34, (coordinate_5 as f32));
        spline_values_6 = [0_f32];
        coordinate_6 = (buffers.minecraft_realism_extreme_hills_slope_height_d5x1x5_output[index(p) as usize] as f32);
        spline_index_6 = binary_search(spline_coordinates_35, (coordinate_6 as f32));
        spline_values_7 = [advanced_hermite(spline_coordinates_35, spline_values_6, spline_derivatives_35, (coordinate_6 as f32), (spline_index_6 as i32)), advanced_hermite(spline_coordinates_33, spline_values_4, spline_derivatives_33, (coordinate_4 as f32), (spline_index_4 as i32)), advanced_hermite(spline_coordinates_31, spline_values_2, spline_derivatives_31, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_29, spline_values, spline_derivatives_29, (coordinate as f32), (spline_index as i32))];
        coordinate_7 = (buffers.density_function_Abs_4_d5x1x5_output[index(p) as usize] as f32);
        spline_index_7 = binary_search(spline_coordinates_36, (coordinate_7 as f32));
        spline_values_8 = [0_f32];
        coordinate_8 = (buffers.minecraft_realism_mountains_slope_height_d5x1x5_output[index(p) as usize] as f32);
        spline_index_8 = binary_search(spline_coordinates_37, (coordinate_8 as f32));
        spline_values_9 = [advanced_hermite(spline_coordinates_37, spline_values_8, spline_derivatives_37, (coordinate_8 as f32), (spline_index_8 as i32)), advanced_hermite(spline_coordinates_35, spline_values_6, spline_derivatives_35, (coordinate_6 as f32), (spline_index_6 as i32)), advanced_hermite(spline_coordinates_33, spline_values_4, spline_derivatives_33, (coordinate_4 as f32), (spline_index_4 as i32)), advanced_hermite(spline_coordinates_31, spline_values_2, spline_derivatives_31, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_29, spline_values, spline_derivatives_29, (coordinate as f32), (spline_index as i32))];
        coordinate_9 = (buffers.density_function_Abs_4_d5x1x5_output[index(p) as usize] as f32);
        spline_index_9 = binary_search(spline_coordinates_38, (coordinate_9 as f32));
        spline_values_10 = [0_f32];
        coordinate_10 = (buffers.minecraft_realism_extreme_mountains_slope_height_d5x1x5_output[index(p) as usize] as f32);
        spline_index_10 = binary_search(spline_coordinates_39, (coordinate_10 as f32));
        spline_values_11 = [advanced_hermite(spline_coordinates_39, spline_values_10, spline_derivatives_39, (coordinate_10 as f32), (spline_index_10 as i32)), advanced_hermite(spline_coordinates_37, spline_values_8, spline_derivatives_37, (coordinate_8 as f32), (spline_index_8 as i32)), advanced_hermite(spline_coordinates_35, spline_values_6, spline_derivatives_35, (coordinate_6 as f32), (spline_index_6 as i32)), advanced_hermite(spline_coordinates_33, spline_values_4, spline_derivatives_33, (coordinate_4 as f32), (spline_index_4 as i32)), advanced_hermite(spline_coordinates_31, spline_values_2, spline_derivatives_31, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_29, spline_values, spline_derivatives_29, (coordinate as f32), (spline_index as i32))];
        coordinate_11 = (buffers.density_function_Abs_4_d5x1x5_output[index(p) as usize] as f32);
        spline_index_11 = binary_search(spline_coordinates_40, (coordinate_11 as f32));
        spline_values_12 = [advanced_hermite(spline_coordinates_30, spline_values_1, spline_derivatives_30, (coordinate_1 as f32), (spline_index_1 as i32)), advanced_hermite(spline_coordinates_32, spline_values_3, spline_derivatives_32, (coordinate_3 as f32), (spline_index_3 as i32)), advanced_hermite(spline_coordinates_34, spline_values_5, spline_derivatives_34, (coordinate_5 as f32), (spline_index_5 as i32)), advanced_hermite(spline_coordinates_34, spline_values_5, spline_derivatives_34, (coordinate_5 as f32), (spline_index_5 as i32)), advanced_hermite(spline_coordinates_36, spline_values_7, spline_derivatives_36, (coordinate_7 as f32), (spline_index_7 as i32)), advanced_hermite(spline_coordinates_38, spline_values_9, spline_derivatives_38, (coordinate_9 as f32), (spline_index_9 as i32)), advanced_hermite(spline_coordinates_40, spline_values_11, spline_derivatives_40, (coordinate_11 as f32), (spline_index_11 as i32))];
        coordinate_12 = (buffers.density_function_Abs_38_d5x1x5_output[index(p) as usize] as f32);
        spline_index_12 = binary_search(spline_coordinates_41, (coordinate_12 as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_41, spline_values_12, spline_derivatives_41, (coordinate_12 as f32), (spline_index_12 as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        return spline_result_f64;
    }
    
    #[inline]
    fn minecraft_new_surface_height_unprocessed_rivered_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 1];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_values_1: [f32; 2];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 1];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 2];
        let mut coordinate_3: f32;
        let mut spline_index_3: i32;
        let mut spline_values_4: [f32; 1];
        let mut coordinate_4: f32;
        let mut spline_index_4: i32;
        let mut spline_values_5: [f32; 3];
        let mut coordinate_5: f32;
        let mut spline_index_5: i32;
        let mut spline_values_6: [f32; 1];
        let mut coordinate_6: f32;
        let mut spline_index_6: i32;
        let mut spline_values_7: [f32; 4];
        let mut coordinate_7: f32;
        let mut spline_index_7: i32;
        let mut spline_values_8: [f32; 1];
        let mut coordinate_8: f32;
        let mut spline_index_8: i32;
        let mut spline_values_9: [f32; 5];
        let mut coordinate_9: f32;
        let mut spline_index_9: i32;
        let mut spline_values_10: [f32; 1];
        let mut coordinate_10: f32;
        let mut spline_index_10: i32;
        let mut spline_values_11: [f32; 6];
        let mut coordinate_11: f32;
        let mut spline_index_11: i32;
        let mut spline_values_12: [f32; 7];
        let mut coordinate_12: f32;
        let mut spline_index_12: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [0_f32];
        coordinate = (buffers.density_function_Add_83_d128x1x128_output[index(p) as usize] as f32);
        spline_index = binary_search(spline_coordinates_134, (coordinate as f32));
        spline_values_1 = [-0.9_f32, advanced_hermite(spline_coordinates_134, spline_values, spline_derivatives_134, (coordinate as f32), (spline_index as i32))];
        coordinate_1 = (buffers.density_function_Abs_84_d128x1x128_output[index(p) as usize] as f32);
        spline_index_1 = binary_search(spline_coordinates_135, (coordinate_1 as f32));
        spline_values_2 = [0_f32];
        coordinate_2 = (buffers.density_function_Add_85_d128x1x128_output[index(p) as usize] as f32);
        spline_index_2 = binary_search(spline_coordinates_136, (coordinate_2 as f32));
        spline_values_3 = [advanced_hermite(spline_coordinates_136, spline_values_2, spline_derivatives_136, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_134, spline_values, spline_derivatives_134, (coordinate as f32), (spline_index as i32))];
        coordinate_3 = (buffers.density_function_Abs_84_d128x1x128_output[index(p) as usize] as f32);
        spline_index_3 = binary_search(spline_coordinates_137, (coordinate_3 as f32));
        spline_values_4 = [0_f32];
        coordinate_4 = (buffers.minecraft_realism_hills_slope_height_d128x1x128_output[index(p) as usize] as f32);
        spline_index_4 = binary_search(spline_coordinates_138, (coordinate_4 as f32));
        spline_values_5 = [advanced_hermite(spline_coordinates_138, spline_values_4, spline_derivatives_138, (coordinate_4 as f32), (spline_index_4 as i32)), advanced_hermite(spline_coordinates_136, spline_values_2, spline_derivatives_136, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_134, spline_values, spline_derivatives_134, (coordinate as f32), (spline_index as i32))];
        coordinate_5 = (buffers.density_function_Abs_84_d128x1x128_output[index(p) as usize] as f32);
        spline_index_5 = binary_search(spline_coordinates_139, (coordinate_5 as f32));
        spline_values_6 = [0_f32];
        coordinate_6 = (buffers.minecraft_realism_extreme_hills_slope_height_d128x1x128_output[index(p) as usize] as f32);
        spline_index_6 = binary_search(spline_coordinates_140, (coordinate_6 as f32));
        spline_values_7 = [advanced_hermite(spline_coordinates_140, spline_values_6, spline_derivatives_140, (coordinate_6 as f32), (spline_index_6 as i32)), advanced_hermite(spline_coordinates_138, spline_values_4, spline_derivatives_138, (coordinate_4 as f32), (spline_index_4 as i32)), advanced_hermite(spline_coordinates_136, spline_values_2, spline_derivatives_136, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_134, spline_values, spline_derivatives_134, (coordinate as f32), (spline_index as i32))];
        coordinate_7 = (buffers.density_function_Abs_84_d128x1x128_output[index(p) as usize] as f32);
        spline_index_7 = binary_search(spline_coordinates_141, (coordinate_7 as f32));
        spline_values_8 = [0_f32];
        coordinate_8 = (buffers.minecraft_realism_mountains_slope_height_d128x1x128_output[index(p) as usize] as f32);
        spline_index_8 = binary_search(spline_coordinates_142, (coordinate_8 as f32));
        spline_values_9 = [advanced_hermite(spline_coordinates_142, spline_values_8, spline_derivatives_142, (coordinate_8 as f32), (spline_index_8 as i32)), advanced_hermite(spline_coordinates_140, spline_values_6, spline_derivatives_140, (coordinate_6 as f32), (spline_index_6 as i32)), advanced_hermite(spline_coordinates_138, spline_values_4, spline_derivatives_138, (coordinate_4 as f32), (spline_index_4 as i32)), advanced_hermite(spline_coordinates_136, spline_values_2, spline_derivatives_136, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_134, spline_values, spline_derivatives_134, (coordinate as f32), (spline_index as i32))];
        coordinate_9 = (buffers.density_function_Abs_84_d128x1x128_output[index(p) as usize] as f32);
        spline_index_9 = binary_search(spline_coordinates_143, (coordinate_9 as f32));
        spline_values_10 = [0_f32];
        coordinate_10 = (buffers.minecraft_realism_extreme_mountains_slope_height_d128x1x128_output[index(p) as usize] as f32);
        spline_index_10 = binary_search(spline_coordinates_144, (coordinate_10 as f32));
        spline_values_11 = [advanced_hermite(spline_coordinates_144, spline_values_10, spline_derivatives_144, (coordinate_10 as f32), (spline_index_10 as i32)), advanced_hermite(spline_coordinates_142, spline_values_8, spline_derivatives_142, (coordinate_8 as f32), (spline_index_8 as i32)), advanced_hermite(spline_coordinates_140, spline_values_6, spline_derivatives_140, (coordinate_6 as f32), (spline_index_6 as i32)), advanced_hermite(spline_coordinates_138, spline_values_4, spline_derivatives_138, (coordinate_4 as f32), (spline_index_4 as i32)), advanced_hermite(spline_coordinates_136, spline_values_2, spline_derivatives_136, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_134, spline_values, spline_derivatives_134, (coordinate as f32), (spline_index as i32))];
        coordinate_11 = (buffers.density_function_Abs_84_d128x1x128_output[index(p) as usize] as f32);
        spline_index_11 = binary_search(spline_coordinates_145, (coordinate_11 as f32));
        spline_values_12 = [advanced_hermite(spline_coordinates_135, spline_values_1, spline_derivatives_135, (coordinate_1 as f32), (spline_index_1 as i32)), advanced_hermite(spline_coordinates_137, spline_values_3, spline_derivatives_137, (coordinate_3 as f32), (spline_index_3 as i32)), advanced_hermite(spline_coordinates_139, spline_values_5, spline_derivatives_139, (coordinate_5 as f32), (spline_index_5 as i32)), advanced_hermite(spline_coordinates_139, spline_values_5, spline_derivatives_139, (coordinate_5 as f32), (spline_index_5 as i32)), advanced_hermite(spline_coordinates_141, spline_values_7, spline_derivatives_141, (coordinate_7 as f32), (spline_index_7 as i32)), advanced_hermite(spline_coordinates_143, spline_values_9, spline_derivatives_143, (coordinate_9 as f32), (spline_index_9 as i32)), advanced_hermite(spline_coordinates_145, spline_values_11, spline_derivatives_145, (coordinate_11 as f32), (spline_index_11 as i32))];
        coordinate_12 = (buffers.density_function_Abs_103_d128x1x128_output[index(p) as usize] as f32);
        spline_index_12 = binary_search(spline_coordinates_146, (coordinate_12 as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_146, spline_values_12, spline_derivatives_146, (coordinate_12 as f32), (spline_index_12 as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        return spline_result_f64;
    }
    
    #[inline]
    fn minecraft_caves_underlands_combined_d5x263x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = clamp(((rpos3.y - -256_f64) / 1272_f64), 0_f64, 1_f64);
        return (((var * -2_f64) + 1_f64) + (buffers.minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1.6_f64));
    }
    
    #[inline]
    fn minecraft_new_surface_height_unprocessed_hollowed_d128x1x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 1];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_values_1: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 2];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [0_f32];
        coordinate = (buffers.minecraft_new_surface_height_unprocessed_rivered_d128x1x128_output[index(p) as usize] as f32);
        spline_index = binary_search(spline_coordinates_131, (coordinate as f32));
        spline_values_1 = [0_f32];
        coordinate_1 = (density_function_Add_104(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_1 = binary_search(spline_coordinates_132, (coordinate_1 as f32));
        spline_values_2 = [advanced_hermite(spline_coordinates_131, spline_values, spline_derivatives_131, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_132, spline_values_1, spline_derivatives_132, (coordinate_1 as f32), (spline_index_1 as i32))];
        coordinate_2 = (buffers.minecraft_new_surface_height_unprocessed_rivered_d128x1x128_output[index(p) as usize] as f32);
        spline_index_2 = binary_search(spline_coordinates_133, (coordinate_2 as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_133, spline_values_2, spline_derivatives_133, (coordinate_2 as f32), (spline_index_2 as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        return spline_result_f64;
    }
    
    #[inline]
    fn density_function_Add_104(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 1];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_values_1: [f32; 2];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 1];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 2];
        let mut coordinate_3: f32;
        let mut spline_index_3: i32;
        let mut spline_values_4: [f32; 1];
        let mut coordinate_4: f32;
        let mut spline_index_4: i32;
        let mut spline_values_5: [f32; 3];
        let mut coordinate_5: f32;
        let mut spline_index_5: i32;
        let mut spline_values_6: [f32; 1];
        let mut coordinate_6: f32;
        let mut spline_index_6: i32;
        let mut spline_values_7: [f32; 4];
        let mut coordinate_7: f32;
        let mut spline_index_7: i32;
        let mut spline_values_8: [f32; 1];
        let mut coordinate_8: f32;
        let mut spline_index_8: i32;
        let mut spline_values_9: [f32; 5];
        let mut coordinate_9: f32;
        let mut spline_index_9: i32;
        let mut spline_values_10: [f32; 1];
        let mut coordinate_10: f32;
        let mut spline_index_10: i32;
        let mut spline_values_11: [f32; 6];
        let mut coordinate_11: f32;
        let mut spline_index_11: i32;
        let mut spline_values_12: [f32; 7];
        let mut coordinate_12: f32;
        let mut spline_index_12: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut spline_values_13: [f32; 1];
        let mut coordinate_13: f32;
        let mut spline_index_13: i32;
        let mut spline_values_14: [f32; 2];
        let mut coordinate_14: f32;
        let mut spline_index_14: i32;
        let mut spline_values_15: [f32; 1];
        let mut coordinate_15: f32;
        let mut spline_index_15: i32;
        let mut spline_values_16: [f32; 2];
        let mut coordinate_16: f32;
        let mut spline_index_16: i32;
        let mut spline_values_17: [f32; 1];
        let mut coordinate_17: f32;
        let mut spline_index_17: i32;
        let mut spline_values_18: [f32; 3];
        let mut coordinate_18: f32;
        let mut spline_index_18: i32;
        let mut spline_values_19: [f32; 1];
        let mut coordinate_19: f32;
        let mut spline_index_19: i32;
        let mut spline_values_20: [f32; 4];
        let mut coordinate_20: f32;
        let mut spline_index_20: i32;
        let mut spline_values_21: [f32; 1];
        let mut coordinate_21: f32;
        let mut spline_index_21: i32;
        let mut spline_values_22: [f32; 5];
        let mut coordinate_22: f32;
        let mut spline_index_22: i32;
        let mut spline_values_23: [f32; 1];
        let mut coordinate_23: f32;
        let mut spline_index_23: i32;
        let mut spline_values_24: [f32; 6];
        let mut coordinate_24: f32;
        let mut spline_index_24: i32;
        let mut spline_values_25: [f32; 7];
        let mut coordinate_25: f32;
        let mut spline_index_25: i32;
        let mut spline_result_f32_1: f32;
        let mut spline_result_f64_1: f64;
        let mut spline_values_26: [f32; 2];
        let mut coordinate_26: f32;
        let mut spline_index_26: i32;
        let mut spline_result_f32_2: f32;
        let mut spline_result_f64_2: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [0_f32];
        coordinate = (buffers.density_function_Add_83_d128x1x128_output[index(p) as usize] as f32);
        spline_index = binary_search(spline_coordinates_169, (coordinate as f32));
        spline_values_1 = [-0.9_f32, advanced_hermite(spline_coordinates_169, spline_values, spline_derivatives_169, (coordinate as f32), (spline_index as i32))];
        coordinate_1 = (buffers.density_function_Abs_84_d128x1x128_output[index(p) as usize] as f32);
        spline_index_1 = binary_search(spline_coordinates_170, (coordinate_1 as f32));
        spline_values_2 = [0_f32];
        coordinate_2 = (buffers.density_function_Add_85_d128x1x128_output[index(p) as usize] as f32);
        spline_index_2 = binary_search(spline_coordinates_171, (coordinate_2 as f32));
        spline_values_3 = [advanced_hermite(spline_coordinates_171, spline_values_2, spline_derivatives_171, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_169, spline_values, spline_derivatives_169, (coordinate as f32), (spline_index as i32))];
        coordinate_3 = (buffers.density_function_Abs_84_d128x1x128_output[index(p) as usize] as f32);
        spline_index_3 = binary_search(spline_coordinates_172, (coordinate_3 as f32));
        spline_values_4 = [0_f32];
        coordinate_4 = (buffers.minecraft_realism_hills_slope_height_d128x1x128_output[index(p) as usize] as f32);
        spline_index_4 = binary_search(spline_coordinates_173, (coordinate_4 as f32));
        spline_values_5 = [advanced_hermite(spline_coordinates_173, spline_values_4, spline_derivatives_173, (coordinate_4 as f32), (spline_index_4 as i32)), advanced_hermite(spline_coordinates_171, spline_values_2, spline_derivatives_171, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_169, spline_values, spline_derivatives_169, (coordinate as f32), (spline_index as i32))];
        coordinate_5 = (buffers.density_function_Abs_84_d128x1x128_output[index(p) as usize] as f32);
        spline_index_5 = binary_search(spline_coordinates_174, (coordinate_5 as f32));
        spline_values_6 = [0_f32];
        coordinate_6 = (buffers.minecraft_realism_extreme_hills_slope_height_d128x1x128_output[index(p) as usize] as f32);
        spline_index_6 = binary_search(spline_coordinates_175, (coordinate_6 as f32));
        spline_values_7 = [advanced_hermite(spline_coordinates_175, spline_values_6, spline_derivatives_175, (coordinate_6 as f32), (spline_index_6 as i32)), advanced_hermite(spline_coordinates_173, spline_values_4, spline_derivatives_173, (coordinate_4 as f32), (spline_index_4 as i32)), advanced_hermite(spline_coordinates_171, spline_values_2, spline_derivatives_171, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_169, spline_values, spline_derivatives_169, (coordinate as f32), (spline_index as i32))];
        coordinate_7 = (buffers.density_function_Abs_84_d128x1x128_output[index(p) as usize] as f32);
        spline_index_7 = binary_search(spline_coordinates_176, (coordinate_7 as f32));
        spline_values_8 = [0_f32];
        coordinate_8 = (buffers.minecraft_realism_mountains_slope_height_d128x1x128_output[index(p) as usize] as f32);
        spline_index_8 = binary_search(spline_coordinates_177, (coordinate_8 as f32));
        spline_values_9 = [advanced_hermite(spline_coordinates_177, spline_values_8, spline_derivatives_177, (coordinate_8 as f32), (spline_index_8 as i32)), advanced_hermite(spline_coordinates_175, spline_values_6, spline_derivatives_175, (coordinate_6 as f32), (spline_index_6 as i32)), advanced_hermite(spline_coordinates_173, spline_values_4, spline_derivatives_173, (coordinate_4 as f32), (spline_index_4 as i32)), advanced_hermite(spline_coordinates_171, spline_values_2, spline_derivatives_171, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_169, spline_values, spline_derivatives_169, (coordinate as f32), (spline_index as i32))];
        coordinate_9 = (buffers.density_function_Abs_84_d128x1x128_output[index(p) as usize] as f32);
        spline_index_9 = binary_search(spline_coordinates_178, (coordinate_9 as f32));
        spline_values_10 = [0_f32];
        coordinate_10 = (buffers.minecraft_realism_extreme_mountains_slope_height_d128x1x128_output[index(p) as usize] as f32);
        spline_index_10 = binary_search(spline_coordinates_179, (coordinate_10 as f32));
        spline_values_11 = [advanced_hermite(spline_coordinates_179, spline_values_10, spline_derivatives_179, (coordinate_10 as f32), (spline_index_10 as i32)), advanced_hermite(spline_coordinates_177, spline_values_8, spline_derivatives_177, (coordinate_8 as f32), (spline_index_8 as i32)), advanced_hermite(spline_coordinates_175, spline_values_6, spline_derivatives_175, (coordinate_6 as f32), (spline_index_6 as i32)), advanced_hermite(spline_coordinates_173, spline_values_4, spline_derivatives_173, (coordinate_4 as f32), (spline_index_4 as i32)), advanced_hermite(spline_coordinates_171, spline_values_2, spline_derivatives_171, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_169, spline_values, spline_derivatives_169, (coordinate as f32), (spline_index as i32))];
        coordinate_11 = (buffers.density_function_Abs_84_d128x1x128_output[index(p) as usize] as f32);
        spline_index_11 = binary_search(spline_coordinates_180, (coordinate_11 as f32));
        spline_values_12 = [advanced_hermite(spline_coordinates_170, spline_values_1, spline_derivatives_170, (coordinate_1 as f32), (spline_index_1 as i32)), advanced_hermite(spline_coordinates_172, spline_values_3, spline_derivatives_172, (coordinate_3 as f32), (spline_index_3 as i32)), advanced_hermite(spline_coordinates_174, spline_values_5, spline_derivatives_174, (coordinate_5 as f32), (spline_index_5 as i32)), advanced_hermite(spline_coordinates_174, spline_values_5, spline_derivatives_174, (coordinate_5 as f32), (spline_index_5 as i32)), advanced_hermite(spline_coordinates_176, spline_values_7, spline_derivatives_176, (coordinate_7 as f32), (spline_index_7 as i32)), advanced_hermite(spline_coordinates_178, spline_values_9, spline_derivatives_178, (coordinate_9 as f32), (spline_index_9 as i32)), advanced_hermite(spline_coordinates_180, spline_values_11, spline_derivatives_180, (coordinate_11 as f32), (spline_index_11 as i32))];
        coordinate_12 = (buffers.density_function_Abs_103_d128x1x128_output[index(p) as usize] as f32);
        spline_index_12 = binary_search(spline_coordinates_181, (coordinate_12 as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_181, spline_values_12, spline_derivatives_181, (coordinate_12 as f32), (spline_index_12 as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        spline_values_13 = [0_f32];
        coordinate_13 = (buffers.density_function_Add_83_d128x1x128_output[index(p) as usize] as f32);
        spline_index_13 = binary_search(spline_coordinates_182, (coordinate_13 as f32));
        spline_values_14 = [-0.9_f32, advanced_hermite(spline_coordinates_182, spline_values_13, spline_derivatives_182, (coordinate_13 as f32), (spline_index_13 as i32))];
        coordinate_14 = (buffers.density_function_Abs_84_d128x1x128_output[index(p) as usize] as f32);
        spline_index_14 = binary_search(spline_coordinates_183, (coordinate_14 as f32));
        spline_values_15 = [0_f32];
        coordinate_15 = (buffers.density_function_Add_85_d128x1x128_output[index(p) as usize] as f32);
        spline_index_15 = binary_search(spline_coordinates_184, (coordinate_15 as f32));
        spline_values_16 = [advanced_hermite(spline_coordinates_184, spline_values_15, spline_derivatives_184, (coordinate_15 as f32), (spline_index_15 as i32)), advanced_hermite(spline_coordinates_182, spline_values_13, spline_derivatives_182, (coordinate_13 as f32), (spline_index_13 as i32))];
        coordinate_16 = (buffers.density_function_Abs_84_d128x1x128_output[index(p) as usize] as f32);
        spline_index_16 = binary_search(spline_coordinates_185, (coordinate_16 as f32));
        spline_values_17 = [0_f32];
        coordinate_17 = (buffers.minecraft_realism_hills_slope_height_d128x1x128_output[index(p) as usize] as f32);
        spline_index_17 = binary_search(spline_coordinates_186, (coordinate_17 as f32));
        spline_values_18 = [advanced_hermite(spline_coordinates_186, spline_values_17, spline_derivatives_186, (coordinate_17 as f32), (spline_index_17 as i32)), advanced_hermite(spline_coordinates_184, spline_values_15, spline_derivatives_184, (coordinate_15 as f32), (spline_index_15 as i32)), advanced_hermite(spline_coordinates_182, spline_values_13, spline_derivatives_182, (coordinate_13 as f32), (spline_index_13 as i32))];
        coordinate_18 = (buffers.density_function_Abs_84_d128x1x128_output[index(p) as usize] as f32);
        spline_index_18 = binary_search(spline_coordinates_187, (coordinate_18 as f32));
        spline_values_19 = [0_f32];
        coordinate_19 = (buffers.minecraft_realism_extreme_hills_slope_height_d128x1x128_output[index(p) as usize] as f32);
        spline_index_19 = binary_search(spline_coordinates_188, (coordinate_19 as f32));
        spline_values_20 = [advanced_hermite(spline_coordinates_188, spline_values_19, spline_derivatives_188, (coordinate_19 as f32), (spline_index_19 as i32)), advanced_hermite(spline_coordinates_186, spline_values_17, spline_derivatives_186, (coordinate_17 as f32), (spline_index_17 as i32)), advanced_hermite(spline_coordinates_184, spline_values_15, spline_derivatives_184, (coordinate_15 as f32), (spline_index_15 as i32)), advanced_hermite(spline_coordinates_182, spline_values_13, spline_derivatives_182, (coordinate_13 as f32), (spline_index_13 as i32))];
        coordinate_20 = (buffers.density_function_Abs_84_d128x1x128_output[index(p) as usize] as f32);
        spline_index_20 = binary_search(spline_coordinates_189, (coordinate_20 as f32));
        spline_values_21 = [0_f32];
        coordinate_21 = (buffers.minecraft_realism_mountains_slope_height_d128x1x128_output[index(p) as usize] as f32);
        spline_index_21 = binary_search(spline_coordinates_190, (coordinate_21 as f32));
        spline_values_22 = [advanced_hermite(spline_coordinates_190, spline_values_21, spline_derivatives_190, (coordinate_21 as f32), (spline_index_21 as i32)), advanced_hermite(spline_coordinates_188, spline_values_19, spline_derivatives_188, (coordinate_19 as f32), (spline_index_19 as i32)), advanced_hermite(spline_coordinates_186, spline_values_17, spline_derivatives_186, (coordinate_17 as f32), (spline_index_17 as i32)), advanced_hermite(spline_coordinates_184, spline_values_15, spline_derivatives_184, (coordinate_15 as f32), (spline_index_15 as i32)), advanced_hermite(spline_coordinates_182, spline_values_13, spline_derivatives_182, (coordinate_13 as f32), (spline_index_13 as i32))];
        coordinate_22 = (buffers.density_function_Abs_84_d128x1x128_output[index(p) as usize] as f32);
        spline_index_22 = binary_search(spline_coordinates_191, (coordinate_22 as f32));
        spline_values_23 = [0_f32];
        coordinate_23 = (buffers.minecraft_realism_extreme_mountains_slope_height_d128x1x128_output[index(p) as usize] as f32);
        spline_index_23 = binary_search(spline_coordinates_192, (coordinate_23 as f32));
        spline_values_24 = [advanced_hermite(spline_coordinates_192, spline_values_23, spline_derivatives_192, (coordinate_23 as f32), (spline_index_23 as i32)), advanced_hermite(spline_coordinates_190, spline_values_21, spline_derivatives_190, (coordinate_21 as f32), (spline_index_21 as i32)), advanced_hermite(spline_coordinates_188, spline_values_19, spline_derivatives_188, (coordinate_19 as f32), (spline_index_19 as i32)), advanced_hermite(spline_coordinates_186, spline_values_17, spline_derivatives_186, (coordinate_17 as f32), (spline_index_17 as i32)), advanced_hermite(spline_coordinates_184, spline_values_15, spline_derivatives_184, (coordinate_15 as f32), (spline_index_15 as i32)), advanced_hermite(spline_coordinates_182, spline_values_13, spline_derivatives_182, (coordinate_13 as f32), (spline_index_13 as i32))];
        coordinate_24 = (buffers.density_function_Abs_84_d128x1x128_output[index(p) as usize] as f32);
        spline_index_24 = binary_search(spline_coordinates_193, (coordinate_24 as f32));
        spline_values_25 = [advanced_hermite(spline_coordinates_183, spline_values_14, spline_derivatives_183, (coordinate_14 as f32), (spline_index_14 as i32)), advanced_hermite(spline_coordinates_185, spline_values_16, spline_derivatives_185, (coordinate_16 as f32), (spline_index_16 as i32)), advanced_hermite(spline_coordinates_187, spline_values_18, spline_derivatives_187, (coordinate_18 as f32), (spline_index_18 as i32)), advanced_hermite(spline_coordinates_187, spline_values_18, spline_derivatives_187, (coordinate_18 as f32), (spline_index_18 as i32)), advanced_hermite(spline_coordinates_189, spline_values_20, spline_derivatives_189, (coordinate_20 as f32), (spline_index_20 as i32)), advanced_hermite(spline_coordinates_191, spline_values_22, spline_derivatives_191, (coordinate_22 as f32), (spline_index_22 as i32)), advanced_hermite(spline_coordinates_193, spline_values_24, spline_derivatives_193, (coordinate_24 as f32), (spline_index_24 as i32))];
        coordinate_25 = (buffers.density_function_Abs_103_d128x1x128_output[index(p) as usize] as f32);
        spline_index_25 = binary_search(spline_coordinates_194, (coordinate_25 as f32));
        spline_result_f32_1 = advanced_hermite(spline_coordinates_194, spline_values_25, spline_derivatives_194, (coordinate_25 as f32), (spline_index_25 as i32));
        spline_result_f64_1 = (spline_result_f32_1 as f64);
        spline_values_26 = [0_f32, -1.5_f32];
        coordinate_26 = (buffers.minecraft_new_surface_height_unprocessed_rivered_d128x1x128_output[index(p) as usize] as f32);
        spline_index_26 = binary_search(spline_coordinates_195, (coordinate_26 as f32));
        spline_result_f32_2 = advanced_hermite(spline_coordinates_195, spline_values_26, spline_derivatives_195, (coordinate_26 as f32), (spline_index_26 as i32));
        spline_result_f64_2 = (spline_result_f32_2 as f64);
        return (spline_result_f64 + (spline_result_f64_1 * spline_result_f64_2));
    }
    
    #[inline]
    fn minecraft_new_surface_height_unprocessed_hollowed_d5x1x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 1];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_values_1: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 2];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [0_f32];
        coordinate = (buffers.minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output[index(p) as usize] as f32);
        spline_index = binary_search(spline_coordinates_26, (coordinate as f32));
        spline_values_1 = [0_f32];
        coordinate_1 = (density_function_Add_40(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_1 = binary_search(spline_coordinates_27, (coordinate_1 as f32));
        spline_values_2 = [advanced_hermite(spline_coordinates_26, spline_values, spline_derivatives_26, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_27, spline_values_1, spline_derivatives_27, (coordinate_1 as f32), (spline_index_1 as i32))];
        coordinate_2 = (buffers.minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output[index(p) as usize] as f32);
        spline_index_2 = binary_search(spline_coordinates_28, (coordinate_2 as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_28, spline_values_2, spline_derivatives_28, (coordinate_2 as f32), (spline_index_2 as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        return spline_result_f64;
    }
    
    #[inline]
    fn density_function_Add_40(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 1];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_values_1: [f32; 2];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 1];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 2];
        let mut coordinate_3: f32;
        let mut spline_index_3: i32;
        let mut spline_values_4: [f32; 1];
        let mut coordinate_4: f32;
        let mut spline_index_4: i32;
        let mut spline_values_5: [f32; 3];
        let mut coordinate_5: f32;
        let mut spline_index_5: i32;
        let mut spline_values_6: [f32; 1];
        let mut coordinate_6: f32;
        let mut spline_index_6: i32;
        let mut spline_values_7: [f32; 4];
        let mut coordinate_7: f32;
        let mut spline_index_7: i32;
        let mut spline_values_8: [f32; 1];
        let mut coordinate_8: f32;
        let mut spline_index_8: i32;
        let mut spline_values_9: [f32; 5];
        let mut coordinate_9: f32;
        let mut spline_index_9: i32;
        let mut spline_values_10: [f32; 1];
        let mut coordinate_10: f32;
        let mut spline_index_10: i32;
        let mut spline_values_11: [f32; 6];
        let mut coordinate_11: f32;
        let mut spline_index_11: i32;
        let mut spline_values_12: [f32; 7];
        let mut coordinate_12: f32;
        let mut spline_index_12: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut spline_values_13: [f32; 1];
        let mut coordinate_13: f32;
        let mut spline_index_13: i32;
        let mut spline_values_14: [f32; 2];
        let mut coordinate_14: f32;
        let mut spline_index_14: i32;
        let mut spline_values_15: [f32; 1];
        let mut coordinate_15: f32;
        let mut spline_index_15: i32;
        let mut spline_values_16: [f32; 2];
        let mut coordinate_16: f32;
        let mut spline_index_16: i32;
        let mut spline_values_17: [f32; 1];
        let mut coordinate_17: f32;
        let mut spline_index_17: i32;
        let mut spline_values_18: [f32; 3];
        let mut coordinate_18: f32;
        let mut spline_index_18: i32;
        let mut spline_values_19: [f32; 1];
        let mut coordinate_19: f32;
        let mut spline_index_19: i32;
        let mut spline_values_20: [f32; 4];
        let mut coordinate_20: f32;
        let mut spline_index_20: i32;
        let mut spline_values_21: [f32; 1];
        let mut coordinate_21: f32;
        let mut spline_index_21: i32;
        let mut spline_values_22: [f32; 5];
        let mut coordinate_22: f32;
        let mut spline_index_22: i32;
        let mut spline_values_23: [f32; 1];
        let mut coordinate_23: f32;
        let mut spline_index_23: i32;
        let mut spline_values_24: [f32; 6];
        let mut coordinate_24: f32;
        let mut spline_index_24: i32;
        let mut spline_values_25: [f32; 7];
        let mut coordinate_25: f32;
        let mut spline_index_25: i32;
        let mut spline_result_f32_1: f32;
        let mut spline_result_f64_1: f64;
        let mut spline_values_26: [f32; 2];
        let mut coordinate_26: f32;
        let mut spline_index_26: i32;
        let mut spline_result_f32_2: f32;
        let mut spline_result_f64_2: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [0_f32];
        coordinate = (buffers.density_function_Add_0_d5x1x5_output[index(p) as usize] as f32);
        spline_index = binary_search(spline_coordinates_64, (coordinate as f32));
        spline_values_1 = [-0.9_f32, advanced_hermite(spline_coordinates_64, spline_values, spline_derivatives_64, (coordinate as f32), (spline_index as i32))];
        coordinate_1 = (buffers.density_function_Abs_4_d5x1x5_output[index(p) as usize] as f32);
        spline_index_1 = binary_search(spline_coordinates_65, (coordinate_1 as f32));
        spline_values_2 = [0_f32];
        coordinate_2 = (buffers.density_function_Add_6_d5x1x5_output[index(p) as usize] as f32);
        spline_index_2 = binary_search(spline_coordinates_66, (coordinate_2 as f32));
        spline_values_3 = [advanced_hermite(spline_coordinates_66, spline_values_2, spline_derivatives_66, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_64, spline_values, spline_derivatives_64, (coordinate as f32), (spline_index as i32))];
        coordinate_3 = (buffers.density_function_Abs_4_d5x1x5_output[index(p) as usize] as f32);
        spline_index_3 = binary_search(spline_coordinates_67, (coordinate_3 as f32));
        spline_values_4 = [0_f32];
        coordinate_4 = (buffers.minecraft_realism_hills_slope_height_d5x1x5_output[index(p) as usize] as f32);
        spline_index_4 = binary_search(spline_coordinates_68, (coordinate_4 as f32));
        spline_values_5 = [advanced_hermite(spline_coordinates_68, spline_values_4, spline_derivatives_68, (coordinate_4 as f32), (spline_index_4 as i32)), advanced_hermite(spline_coordinates_66, spline_values_2, spline_derivatives_66, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_64, spline_values, spline_derivatives_64, (coordinate as f32), (spline_index as i32))];
        coordinate_5 = (buffers.density_function_Abs_4_d5x1x5_output[index(p) as usize] as f32);
        spline_index_5 = binary_search(spline_coordinates_69, (coordinate_5 as f32));
        spline_values_6 = [0_f32];
        coordinate_6 = (buffers.minecraft_realism_extreme_hills_slope_height_d5x1x5_output[index(p) as usize] as f32);
        spline_index_6 = binary_search(spline_coordinates_70, (coordinate_6 as f32));
        spline_values_7 = [advanced_hermite(spline_coordinates_70, spline_values_6, spline_derivatives_70, (coordinate_6 as f32), (spline_index_6 as i32)), advanced_hermite(spline_coordinates_68, spline_values_4, spline_derivatives_68, (coordinate_4 as f32), (spline_index_4 as i32)), advanced_hermite(spline_coordinates_66, spline_values_2, spline_derivatives_66, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_64, spline_values, spline_derivatives_64, (coordinate as f32), (spline_index as i32))];
        coordinate_7 = (buffers.density_function_Abs_4_d5x1x5_output[index(p) as usize] as f32);
        spline_index_7 = binary_search(spline_coordinates_71, (coordinate_7 as f32));
        spline_values_8 = [0_f32];
        coordinate_8 = (buffers.minecraft_realism_mountains_slope_height_d5x1x5_output[index(p) as usize] as f32);
        spline_index_8 = binary_search(spline_coordinates_72, (coordinate_8 as f32));
        spline_values_9 = [advanced_hermite(spline_coordinates_72, spline_values_8, spline_derivatives_72, (coordinate_8 as f32), (spline_index_8 as i32)), advanced_hermite(spline_coordinates_70, spline_values_6, spline_derivatives_70, (coordinate_6 as f32), (spline_index_6 as i32)), advanced_hermite(spline_coordinates_68, spline_values_4, spline_derivatives_68, (coordinate_4 as f32), (spline_index_4 as i32)), advanced_hermite(spline_coordinates_66, spline_values_2, spline_derivatives_66, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_64, spline_values, spline_derivatives_64, (coordinate as f32), (spline_index as i32))];
        coordinate_9 = (buffers.density_function_Abs_4_d5x1x5_output[index(p) as usize] as f32);
        spline_index_9 = binary_search(spline_coordinates_73, (coordinate_9 as f32));
        spline_values_10 = [0_f32];
        coordinate_10 = (buffers.minecraft_realism_extreme_mountains_slope_height_d5x1x5_output[index(p) as usize] as f32);
        spline_index_10 = binary_search(spline_coordinates_74, (coordinate_10 as f32));
        spline_values_11 = [advanced_hermite(spline_coordinates_74, spline_values_10, spline_derivatives_74, (coordinate_10 as f32), (spline_index_10 as i32)), advanced_hermite(spline_coordinates_72, spline_values_8, spline_derivatives_72, (coordinate_8 as f32), (spline_index_8 as i32)), advanced_hermite(spline_coordinates_70, spline_values_6, spline_derivatives_70, (coordinate_6 as f32), (spline_index_6 as i32)), advanced_hermite(spline_coordinates_68, spline_values_4, spline_derivatives_68, (coordinate_4 as f32), (spline_index_4 as i32)), advanced_hermite(spline_coordinates_66, spline_values_2, spline_derivatives_66, (coordinate_2 as f32), (spline_index_2 as i32)), advanced_hermite(spline_coordinates_64, spline_values, spline_derivatives_64, (coordinate as f32), (spline_index as i32))];
        coordinate_11 = (buffers.density_function_Abs_4_d5x1x5_output[index(p) as usize] as f32);
        spline_index_11 = binary_search(spline_coordinates_75, (coordinate_11 as f32));
        spline_values_12 = [advanced_hermite(spline_coordinates_65, spline_values_1, spline_derivatives_65, (coordinate_1 as f32), (spline_index_1 as i32)), advanced_hermite(spline_coordinates_67, spline_values_3, spline_derivatives_67, (coordinate_3 as f32), (spline_index_3 as i32)), advanced_hermite(spline_coordinates_69, spline_values_5, spline_derivatives_69, (coordinate_5 as f32), (spline_index_5 as i32)), advanced_hermite(spline_coordinates_69, spline_values_5, spline_derivatives_69, (coordinate_5 as f32), (spline_index_5 as i32)), advanced_hermite(spline_coordinates_71, spline_values_7, spline_derivatives_71, (coordinate_7 as f32), (spline_index_7 as i32)), advanced_hermite(spline_coordinates_73, spline_values_9, spline_derivatives_73, (coordinate_9 as f32), (spline_index_9 as i32)), advanced_hermite(spline_coordinates_75, spline_values_11, spline_derivatives_75, (coordinate_11 as f32), (spline_index_11 as i32))];
        coordinate_12 = (buffers.density_function_Abs_38_d5x1x5_output[index(p) as usize] as f32);
        spline_index_12 = binary_search(spline_coordinates_76, (coordinate_12 as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_76, spline_values_12, spline_derivatives_76, (coordinate_12 as f32), (spline_index_12 as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        spline_values_13 = [0_f32];
        coordinate_13 = (buffers.density_function_Add_0_d5x1x5_output[index(p) as usize] as f32);
        spline_index_13 = binary_search(spline_coordinates_77, (coordinate_13 as f32));
        spline_values_14 = [-0.9_f32, advanced_hermite(spline_coordinates_77, spline_values_13, spline_derivatives_77, (coordinate_13 as f32), (spline_index_13 as i32))];
        coordinate_14 = (buffers.density_function_Abs_4_d5x1x5_output[index(p) as usize] as f32);
        spline_index_14 = binary_search(spline_coordinates_78, (coordinate_14 as f32));
        spline_values_15 = [0_f32];
        coordinate_15 = (buffers.density_function_Add_6_d5x1x5_output[index(p) as usize] as f32);
        spline_index_15 = binary_search(spline_coordinates_79, (coordinate_15 as f32));
        spline_values_16 = [advanced_hermite(spline_coordinates_79, spline_values_15, spline_derivatives_79, (coordinate_15 as f32), (spline_index_15 as i32)), advanced_hermite(spline_coordinates_77, spline_values_13, spline_derivatives_77, (coordinate_13 as f32), (spline_index_13 as i32))];
        coordinate_16 = (buffers.density_function_Abs_4_d5x1x5_output[index(p) as usize] as f32);
        spline_index_16 = binary_search(spline_coordinates_80, (coordinate_16 as f32));
        spline_values_17 = [0_f32];
        coordinate_17 = (buffers.minecraft_realism_hills_slope_height_d5x1x5_output[index(p) as usize] as f32);
        spline_index_17 = binary_search(spline_coordinates_81, (coordinate_17 as f32));
        spline_values_18 = [advanced_hermite(spline_coordinates_81, spline_values_17, spline_derivatives_81, (coordinate_17 as f32), (spline_index_17 as i32)), advanced_hermite(spline_coordinates_79, spline_values_15, spline_derivatives_79, (coordinate_15 as f32), (spline_index_15 as i32)), advanced_hermite(spline_coordinates_77, spline_values_13, spline_derivatives_77, (coordinate_13 as f32), (spline_index_13 as i32))];
        coordinate_18 = (buffers.density_function_Abs_4_d5x1x5_output[index(p) as usize] as f32);
        spline_index_18 = binary_search(spline_coordinates_82, (coordinate_18 as f32));
        spline_values_19 = [0_f32];
        coordinate_19 = (buffers.minecraft_realism_extreme_hills_slope_height_d5x1x5_output[index(p) as usize] as f32);
        spline_index_19 = binary_search(spline_coordinates_83, (coordinate_19 as f32));
        spline_values_20 = [advanced_hermite(spline_coordinates_83, spline_values_19, spline_derivatives_83, (coordinate_19 as f32), (spline_index_19 as i32)), advanced_hermite(spline_coordinates_81, spline_values_17, spline_derivatives_81, (coordinate_17 as f32), (spline_index_17 as i32)), advanced_hermite(spline_coordinates_79, spline_values_15, spline_derivatives_79, (coordinate_15 as f32), (spline_index_15 as i32)), advanced_hermite(spline_coordinates_77, spline_values_13, spline_derivatives_77, (coordinate_13 as f32), (spline_index_13 as i32))];
        coordinate_20 = (buffers.density_function_Abs_4_d5x1x5_output[index(p) as usize] as f32);
        spline_index_20 = binary_search(spline_coordinates_84, (coordinate_20 as f32));
        spline_values_21 = [0_f32];
        coordinate_21 = (buffers.minecraft_realism_mountains_slope_height_d5x1x5_output[index(p) as usize] as f32);
        spline_index_21 = binary_search(spline_coordinates_85, (coordinate_21 as f32));
        spline_values_22 = [advanced_hermite(spline_coordinates_85, spline_values_21, spline_derivatives_85, (coordinate_21 as f32), (spline_index_21 as i32)), advanced_hermite(spline_coordinates_83, spline_values_19, spline_derivatives_83, (coordinate_19 as f32), (spline_index_19 as i32)), advanced_hermite(spline_coordinates_81, spline_values_17, spline_derivatives_81, (coordinate_17 as f32), (spline_index_17 as i32)), advanced_hermite(spline_coordinates_79, spline_values_15, spline_derivatives_79, (coordinate_15 as f32), (spline_index_15 as i32)), advanced_hermite(spline_coordinates_77, spline_values_13, spline_derivatives_77, (coordinate_13 as f32), (spline_index_13 as i32))];
        coordinate_22 = (buffers.density_function_Abs_4_d5x1x5_output[index(p) as usize] as f32);
        spline_index_22 = binary_search(spline_coordinates_86, (coordinate_22 as f32));
        spline_values_23 = [0_f32];
        coordinate_23 = (buffers.minecraft_realism_extreme_mountains_slope_height_d5x1x5_output[index(p) as usize] as f32);
        spline_index_23 = binary_search(spline_coordinates_87, (coordinate_23 as f32));
        spline_values_24 = [advanced_hermite(spline_coordinates_87, spline_values_23, spline_derivatives_87, (coordinate_23 as f32), (spline_index_23 as i32)), advanced_hermite(spline_coordinates_85, spline_values_21, spline_derivatives_85, (coordinate_21 as f32), (spline_index_21 as i32)), advanced_hermite(spline_coordinates_83, spline_values_19, spline_derivatives_83, (coordinate_19 as f32), (spline_index_19 as i32)), advanced_hermite(spline_coordinates_81, spline_values_17, spline_derivatives_81, (coordinate_17 as f32), (spline_index_17 as i32)), advanced_hermite(spline_coordinates_79, spline_values_15, spline_derivatives_79, (coordinate_15 as f32), (spline_index_15 as i32)), advanced_hermite(spline_coordinates_77, spline_values_13, spline_derivatives_77, (coordinate_13 as f32), (spline_index_13 as i32))];
        coordinate_24 = (buffers.density_function_Abs_4_d5x1x5_output[index(p) as usize] as f32);
        spline_index_24 = binary_search(spline_coordinates_88, (coordinate_24 as f32));
        spline_values_25 = [advanced_hermite(spline_coordinates_78, spline_values_14, spline_derivatives_78, (coordinate_14 as f32), (spline_index_14 as i32)), advanced_hermite(spline_coordinates_80, spline_values_16, spline_derivatives_80, (coordinate_16 as f32), (spline_index_16 as i32)), advanced_hermite(spline_coordinates_82, spline_values_18, spline_derivatives_82, (coordinate_18 as f32), (spline_index_18 as i32)), advanced_hermite(spline_coordinates_82, spline_values_18, spline_derivatives_82, (coordinate_18 as f32), (spline_index_18 as i32)), advanced_hermite(spline_coordinates_84, spline_values_20, spline_derivatives_84, (coordinate_20 as f32), (spline_index_20 as i32)), advanced_hermite(spline_coordinates_86, spline_values_22, spline_derivatives_86, (coordinate_22 as f32), (spline_index_22 as i32)), advanced_hermite(spline_coordinates_88, spline_values_24, spline_derivatives_88, (coordinate_24 as f32), (spline_index_24 as i32))];
        coordinate_25 = (buffers.density_function_Abs_38_d5x1x5_output[index(p) as usize] as f32);
        spline_index_25 = binary_search(spline_coordinates_89, (coordinate_25 as f32));
        spline_result_f32_1 = advanced_hermite(spline_coordinates_89, spline_values_25, spline_derivatives_89, (coordinate_25 as f32), (spline_index_25 as i32));
        spline_result_f64_1 = (spline_result_f32_1 as f64);
        spline_values_26 = [0_f32, -1.5_f32];
        coordinate_26 = (buffers.minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output[index(p) as usize] as f32);
        spline_index_26 = binary_search(spline_coordinates_90, (coordinate_26 as f32));
        spline_result_f32_2 = advanced_hermite(spline_coordinates_90, spline_values_26, spline_derivatives_90, (coordinate_26 as f32), (spline_index_26 as i32));
        spline_result_f64_2 = (spline_result_f32_2 as f64);
        return (spline_result_f64 + (spline_result_f64_1 * spline_result_f64_2));
    }
    
    #[inline]
    fn density_function_109_d128x32x128(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = clamp(((rpos3.y - -64_f64) / 2096_f64), 0_f64, 1_f64);
        return (-0.15_f64 + (((var * -2_f64) + 1_f64) + buffers.minecraft_new_surface_height_unprocessed_hollowed_d128x1x128_output[flat_y_zero_index(pos3, 128_i32, 128_i32) as usize]));
    }
    
    #[inline]
    fn minecraft_new_surface_combined_processed_d5x263x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = clamp(((rpos3.y - -64_f64) / 2096_f64), 0_f64, 1_f64);
        return (((var * -2_f64) + 1_f64) + buffers.minecraft_new_surface_height_unprocessed_hollowed_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize]);
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
        var = ((floor((2032_f64 / 64_f64)) * 64_f64) as i32);
        if (var <= -64_i32) {
            var = -64_i32;
        } else {
            var_1 = (var / 64_i32);
            var = -64_i32;
            while (var_1 >= -1_i32) {
                if (buffers.density_function_109_d128x32x128_output[preliminary_surface_index(pos3, ((var_1 - -1_i32) as i32), 64_i32, 128_i32, 32_i32, 128_i32) as usize] > 0_f64) {
                    var = (var_1 * 64_i32);
                    break;
                }
                var_1 = (var_1 - 1_i32);
            }
        }
        var_2 = (var as f64);
        return var_2;
    }
    
    #[inline]
    fn minecraft_new_combination_fixed_d5x263x5(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 1];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_values_1: [f32; 2];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [0_f32];
        coordinate = (minecraft_new_combination_unfixed_new(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index = binary_search(spline_coordinates, (coordinate as f32));
        spline_values_1 = [0.1_f32, advanced_hermite(spline_coordinates, spline_values, spline_derivatives, (coordinate as f32), (spline_index as i32))];
        coordinate_1 = (density_function_YClampedGradient_76(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_1 = binary_search(spline_coordinates_1, (coordinate_1 as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_1, spline_values_1, spline_derivatives_1, (coordinate_1 as f32), (spline_index_1 as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        return spline_result_f64;
    }
    
    #[inline]
    fn minecraft_new_combination_unfixed_new(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 1];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_values_1: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 2];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut spline_values_3: [f32; 1];
        let mut coordinate_3: f32;
        let mut spline_index_3: i32;
        let mut spline_values_4: [f32; 1];
        let mut coordinate_4: f32;
        let mut spline_index_4: i32;
        let mut spline_values_5: [f32; 2];
        let mut coordinate_5: f32;
        let mut spline_index_5: i32;
        let mut spline_values_6: [f32; 1];
        let mut coordinate_6: f32;
        let mut spline_index_6: i32;
        let mut spline_values_7: [f32; 2];
        let mut coordinate_7: f32;
        let mut spline_index_7: i32;
        let mut spline_values_8: [f32; 1];
        let mut coordinate_8: f32;
        let mut spline_index_8: i32;
        let mut spline_values_9: [f32; 2];
        let mut coordinate_9: f32;
        let mut spline_index_9: i32;
        let mut spline_values_10: [f32; 1];
        let mut coordinate_10: f32;
        let mut spline_index_10: i32;
        let mut spline_values_11: [f32; 4];
        let mut coordinate_11: f32;
        let mut spline_index_11: i32;
        let mut spline_values_12: [f32; 1];
        let mut coordinate_12: f32;
        let mut spline_index_12: i32;
        let mut spline_values_13: [f32; 2];
        let mut coordinate_13: f32;
        let mut spline_index_13: i32;
        let mut spline_values_14: [f32; 1];
        let mut coordinate_14: f32;
        let mut spline_index_14: i32;
        let mut spline_values_15: [f32; 2];
        let mut coordinate_15: f32;
        let mut spline_index_15: i32;
        let mut spline_values_16: [f32; 1];
        let mut coordinate_16: f32;
        let mut spline_index_16: i32;
        let mut spline_values_17: [f32; 2];
        let mut coordinate_17: f32;
        let mut spline_index_17: i32;
        let mut spline_values_18: [f32; 4];
        let mut coordinate_18: f32;
        let mut spline_index_18: i32;
        let mut spline_values_19: [f32; 2];
        let mut coordinate_19: f32;
        let mut spline_index_19: i32;
        let mut spline_values_20: [f32; 2];
        let mut coordinate_20: f32;
        let mut spline_index_20: i32;
        let mut spline_result_f32_1: f32;
        let mut spline_result_f64_1: f64;
        let mut inlined_density_86: f64;
        let mut inlined_density_87: f64;
        let mut inlined_density_88: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        inlined_density_88 = minecraft_new_surface_combined_processed_base(p, config, buffers, auto_constants, host_consts);
        inlined_density_87 = minecraft_caves_barrier_2(p, config, buffers, auto_constants, host_consts);
        inlined_density_86 = minecraft_new_combination_unfixed_2(p, config, buffers, auto_constants, host_consts);
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [0_f32];
        coordinate = (minecraft_new_combination_unfixed_3(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index = binary_search(spline_coordinates_2, (coordinate as f32));
        spline_values_1 = [0_f32];
        coordinate_1 = (inlined_density_86 as f32);
        spline_index_1 = binary_search(spline_coordinates_3, (coordinate_1 as f32));
        spline_values_2 = [advanced_hermite(spline_coordinates_2, spline_values, spline_derivatives_2, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_3, spline_values_1, spline_derivatives_3, (coordinate_1 as f32), (spline_index_1 as i32))];
        coordinate_2 = (minecraft_new_surface_height_processed_base(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_2 = binary_search(spline_coordinates_4, (coordinate_2 as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_4, spline_values_2, spline_derivatives_4, (coordinate_2 as f32), (spline_index_2 as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        spline_values_3 = [0_f32];
        coordinate_3 = (inlined_density_86 as f32);
        spline_index_3 = binary_search(spline_coordinates_5, (coordinate_3 as f32));
        spline_values_4 = [0_f32];
        coordinate_4 = (minecraft_caves_old_small(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_4 = binary_search(spline_coordinates_6, (coordinate_4 as f32));
        spline_values_5 = [0.1_f32, advanced_hermite(spline_coordinates_6, spline_values_4, spline_derivatives_6, (coordinate_4 as f32), (spline_index_4 as i32))];
        coordinate_5 = (inlined_density_87 as f32);
        spline_index_5 = binary_search(spline_coordinates_7, (coordinate_5 as f32));
        spline_values_6 = [0_f32];
        coordinate_6 = (minecraft_caves_old_medium(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_6 = binary_search(spline_coordinates_8, (coordinate_6 as f32));
        spline_values_7 = [0.1_f32, advanced_hermite(spline_coordinates_8, spline_values_6, spline_derivatives_8, (coordinate_6 as f32), (spline_index_6 as i32))];
        coordinate_7 = (inlined_density_87 as f32);
        spline_index_7 = binary_search(spline_coordinates_9, (coordinate_7 as f32));
        spline_values_8 = [0_f32];
        coordinate_8 = (minecraft_caves_old_large(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_8 = binary_search(spline_coordinates_10, (coordinate_8 as f32));
        spline_values_9 = [0.1_f32, advanced_hermite(spline_coordinates_10, spline_values_8, spline_derivatives_10, (coordinate_8 as f32), (spline_index_8 as i32))];
        coordinate_9 = (inlined_density_87 as f32);
        spline_index_9 = binary_search(spline_coordinates_11, (coordinate_9 as f32));
        spline_values_10 = [0_f32];
        coordinate_10 = (buffers.minecraft_caves_underlands_combined_d5x263x5_output[index(p) as usize] as f32);
        spline_index_10 = binary_search(spline_coordinates_12, (coordinate_10 as f32));
        spline_values_11 = [advanced_hermite(spline_coordinates_7, spline_values_5, spline_derivatives_7, (coordinate_5 as f32), (spline_index_5 as i32)), advanced_hermite(spline_coordinates_9, spline_values_7, spline_derivatives_9, (coordinate_7 as f32), (spline_index_7 as i32)), advanced_hermite(spline_coordinates_11, spline_values_9, spline_derivatives_11, (coordinate_9 as f32), (spline_index_9 as i32)), advanced_hermite(spline_coordinates_12, spline_values_10, spline_derivatives_12, (coordinate_10 as f32), (spline_index_10 as i32))];
        coordinate_11 = (inlined_density_88 as f32);
        spline_index_11 = binary_search(spline_coordinates_13, (coordinate_11 as f32));
        spline_values_12 = [0_f32];
        coordinate_12 = (minecraft_caves_new_small(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_12 = binary_search(spline_coordinates_14, (coordinate_12 as f32));
        spline_values_13 = [0.1_f32, advanced_hermite(spline_coordinates_14, spline_values_12, spline_derivatives_14, (coordinate_12 as f32), (spline_index_12 as i32))];
        coordinate_13 = (inlined_density_87 as f32);
        spline_index_13 = binary_search(spline_coordinates_15, (coordinate_13 as f32));
        spline_values_14 = [0_f32];
        coordinate_14 = (minecraft_caves_new_medium(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_14 = binary_search(spline_coordinates_16, (coordinate_14 as f32));
        spline_values_15 = [0.1_f32, advanced_hermite(spline_coordinates_16, spline_values_14, spline_derivatives_16, (coordinate_14 as f32), (spline_index_14 as i32))];
        coordinate_15 = (inlined_density_87 as f32);
        spline_index_15 = binary_search(spline_coordinates_17, (coordinate_15 as f32));
        spline_values_16 = [0_f32];
        coordinate_16 = (minecraft_caves_new_large(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_16 = binary_search(spline_coordinates_18, (coordinate_16 as f32));
        spline_values_17 = [0.1_f32, advanced_hermite(spline_coordinates_18, spline_values_16, spline_derivatives_18, (coordinate_16 as f32), (spline_index_16 as i32))];
        coordinate_17 = (inlined_density_87 as f32);
        spline_index_17 = binary_search(spline_coordinates_19, (coordinate_17 as f32));
        spline_values_18 = [advanced_hermite(spline_coordinates_15, spline_values_13, spline_derivatives_15, (coordinate_13 as f32), (spline_index_13 as i32)), advanced_hermite(spline_coordinates_17, spline_values_15, spline_derivatives_17, (coordinate_15 as f32), (spline_index_15 as i32)), advanced_hermite(spline_coordinates_19, spline_values_17, spline_derivatives_19, (coordinate_17 as f32), (spline_index_17 as i32)), advanced_hermite(spline_coordinates_12, spline_values_10, spline_derivatives_12, (coordinate_10 as f32), (spline_index_10 as i32))];
        coordinate_18 = (inlined_density_88 as f32);
        spline_index_18 = binary_search(spline_coordinates_20, (coordinate_18 as f32));
        spline_values_19 = [advanced_hermite(spline_coordinates_13, spline_values_11, spline_derivatives_13, (coordinate_11 as f32), (spline_index_11 as i32)), advanced_hermite(spline_coordinates_20, spline_values_18, spline_derivatives_20, (coordinate_18 as f32), (spline_index_18 as i32))];
        coordinate_19 = (minecraft_caves_barrier_1(p, config, buffers, auto_constants, host_consts) as f32);
        spline_index_19 = binary_search(spline_coordinates_21, (coordinate_19 as f32));
        spline_values_20 = [advanced_hermite(spline_coordinates_5, spline_values_3, spline_derivatives_5, (coordinate_3 as f32), (spline_index_3 as i32)), advanced_hermite(spline_coordinates_21, spline_values_19, spline_derivatives_21, (coordinate_19 as f32), (spline_index_19 as i32))];
        coordinate_20 = (buffers.minecraft_new_surface_combined_processed_d5x263x5_output[index(p) as usize] as f32);
        spline_index_20 = binary_search(spline_coordinates_22, (coordinate_20 as f32));
        spline_result_f32_1 = advanced_hermite(spline_coordinates_22, spline_values_20, spline_derivatives_22, (coordinate_20 as f32), (spline_index_20 as i32));
        spline_result_f64_1 = (spline_result_f32_1 as f64);
        return min(spline_result_f64, spline_result_f64_1);
    }
    
    #[inline]
    fn minecraft_caves_barrier_1(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_cave_layer_75_d5x263x5_output[index(p) as usize];
    }
    
    #[inline]
    fn minecraft_caves_new_large(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 41];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32];
        coordinate = (buffers.minecraft_determiner_cave_1_d5x263x5_output[index(p) as usize] as f32);
        spline_index = binary_search(spline_coordinates_130, (coordinate as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_130, spline_values, spline_derivatives_130, (coordinate as f32), (spline_index as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        return (spline_result_f64 + ((buffers.minecraft_cave_layer_74_d5x263x5_output[index(p) as usize] * 1.5_f64) + (buffers.minecraft_jagged_18_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.5_f64)));
    }
    
    #[inline]
    fn minecraft_caves_new_medium(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 81];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32];
        coordinate = (buffers.minecraft_determiner_cave_1_d5x263x5_output[index(p) as usize] as f32);
        spline_index = binary_search(spline_coordinates_129, (coordinate as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_129, spline_values, spline_derivatives_129, (coordinate as f32), (spline_index as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        return (spline_result_f64 + ((buffers.minecraft_cave_layer_73_d5x263x5_output[index(p) as usize] * 1.5_f64) + (buffers.minecraft_jagged_9_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.5_f64)));
    }
    
    #[inline]
    fn minecraft_caves_new_small(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 165];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32];
        coordinate = (buffers.minecraft_determiner_cave_1_d5x263x5_output[index(p) as usize] as f32);
        spline_index = binary_search(spline_coordinates_128, (coordinate as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_128, spline_values, spline_derivatives_128, (coordinate as f32), (spline_index as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        return (spline_result_f64 + ((buffers.minecraft_cave_layer_71_d5x263x5_output[index(p) as usize] * 1.5_f64) + (buffers.minecraft_jagged_72_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.5_f64)));
    }
    
    #[inline]
    fn minecraft_caves_old_large(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_cave_layer_69_d5x263x5_output[index(p) as usize];
    }
    
    #[inline]
    fn minecraft_caves_old_medium(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_cave_layer_68_d5x263x5_output[index(p) as usize];
    }
    
    #[inline]
    fn minecraft_caves_old_small(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_cave_layer_66_d5x263x5_output[index(p) as usize];
    }
    
    #[inline]
    fn minecraft_new_surface_height_processed_base(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
    }
    
    #[inline]
    fn minecraft_new_combination_unfixed_3(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 1];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_values_1: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 2];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [0_f32];
        coordinate = (buffers.minecraft_new_surface_combined_processed_d5x263x5_output[index(p) as usize] as f32);
        spline_index = binary_search(spline_coordinates_23, (coordinate as f32));
        spline_values_1 = [0_f32];
        coordinate_1 = (buffers.minecraft_caves_underlands_combined_d5x263x5_output[index(p) as usize] as f32);
        spline_index_1 = binary_search(spline_coordinates_24, (coordinate_1 as f32));
        spline_values_2 = [advanced_hermite(spline_coordinates_23, spline_values, spline_derivatives_23, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_24, spline_values_1, spline_derivatives_24, (coordinate_1 as f32), (spline_index_1 as i32))];
        coordinate_2 = (buffers.minecraft_new_surface_combined_unprocessed_base_d5x263x5_output[index(p) as usize] as f32);
        spline_index_2 = binary_search(spline_coordinates_25, (coordinate_2 as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_25, spline_values_2, spline_derivatives_25, (coordinate_2 as f32), (spline_index_2 as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        return spline_result_f64;
    }
    
    #[inline]
    fn minecraft_new_combination_unfixed_2(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut spline_values: [f32; 1];
        let mut coordinate: f32;
        let mut spline_index: i32;
        let mut spline_values_1: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_1: i32;
        let mut spline_values_2: [f32; 2];
        let mut coordinate_2: f32;
        let mut spline_index_2: i32;
        let mut spline_result_f32: f32;
        let mut spline_result_f64: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        spline_values = [0_f32];
        coordinate = (buffers.minecraft_new_surface_combined_processed_d5x263x5_output[index(p) as usize] as f32);
        spline_index = binary_search(spline_coordinates_125, (coordinate as f32));
        spline_values_1 = [0_f32];
        coordinate_1 = (buffers.minecraft_caves_underlands_combined_d5x263x5_output[index(p) as usize] as f32);
        spline_index_1 = binary_search(spline_coordinates_126, (coordinate_1 as f32));
        spline_values_2 = [advanced_hermite(spline_coordinates_125, spline_values, spline_derivatives_125, (coordinate as f32), (spline_index as i32)), advanced_hermite(spline_coordinates_126, spline_values_1, spline_derivatives_126, (coordinate_1 as f32), (spline_index_1 as i32))];
        coordinate_2 = (buffers.minecraft_new_surface_combined_unprocessed_base_d5x263x5_output[index(p) as usize] as f32);
        spline_index_2 = binary_search(spline_coordinates_127, (coordinate_2 as f32));
        spline_result_f32 = advanced_hermite(spline_coordinates_127, spline_values_2, spline_derivatives_127, (coordinate_2 as f32), (spline_index_2 as i32));
        spline_result_f64 = (spline_result_f32 as f64);
        return spline_result_f64;
    }
    
    #[inline]
    fn minecraft_caves_barrier_2(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return minecraft_cave_layer_67(p, config, buffers, auto_constants, host_consts);
    }
    
    #[inline]
    fn minecraft_cave_layer_67(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = (origin + (pos3 * Vec3::new(4_f64, 8_f64, 4_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), (&auto_constants.minecraft_cave_layer_0_octave__8)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), (&auto_constants.minecraft_cave_layer_1_octave__8)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn minecraft_new_surface_combined_processed_base(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = clamp(((rpos3.y - -64_f64) / 2096_f64), 0_f64, 1_f64);
        return (((var * -2_f64) + 1_f64) + buffers.minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize]);
    }
    
    #[inline]
    fn density_function_YClampedGradient_76(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut var: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        var = clamp(((rpos3.y - -64_f64) / 2096_f64), 0_f64, 1_f64);
        return ((var * 2096_f64) + -64_f64);
    }
    
    #[inline]
    fn final_density_with_beardify_d16x2096x16(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return (interpolate(buffers.minecraft_new_combination_fixed_d5x263x5_output[cornerx0y0z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.minecraft_new_combination_fixed_d5x263x5_output[cornerx4y0z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.minecraft_new_combination_fixed_d5x263x5_output[cornerx0y8z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.minecraft_new_combination_fixed_d5x263x5_output[cornerx4y8z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.minecraft_new_combination_fixed_d5x263x5_output[cornerx0y0z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.minecraft_new_combination_fixed_d5x263x5_output[cornerx4y0z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.minecraft_new_combination_fixed_d5x263x5_output[cornerx0y8z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.minecraft_new_combination_fixed_d5x263x5_output[cornerx4y8z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], xfract4(pos3), yfract8(pos3), zfract4(pos3)) + Beardifier(p, config, buffers, auto_constants, host_consts));
    }
    
    #[inline]
    fn Beardifier(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
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
    fn final_density_d16x2096x16(p: PositionIterator, config: &Config, buffers: &InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return interpolate(buffers.minecraft_new_combination_fixed_d5x263x5_output[cornerx0y0z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.minecraft_new_combination_fixed_d5x263x5_output[cornerx4y0z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.minecraft_new_combination_fixed_d5x263x5_output[cornerx0y8z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.minecraft_new_combination_fixed_d5x263x5_output[cornerx4y8z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.minecraft_new_combination_fixed_d5x263x5_output[cornerx0y0z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.minecraft_new_combination_fixed_d5x263x5_output[cornerx4y0z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.minecraft_new_combination_fixed_d5x263x5_output[cornerx0y8z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], buffers.minecraft_new_combination_fixed_d5x263x5_output[cornerx4y8z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], xfract4(pos3), yfract8(pos3), zfract4(pos3));
    }
    
    pub fn minecraft_cave_layer_66_d5x263x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(8_f64, 8_f64, 8_f64), position_scale: Vec3::new(32_f64, 64_f64, 32_f64), dimensions: (5_i32, 263_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_cave_layer_66_d5x263x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_cave_layer_66_d5x263x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_cave_layer_68_d5x263x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(6_f64, 6_f64, 6_f64), position_scale: Vec3::new(24_f64, 48_f64, 24_f64), dimensions: (5_i32, 263_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_cave_layer_68_d5x263x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_cave_layer_68_d5x263x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_cave_layer_71_d5x263x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(10_f64, 4_f64, 10_f64), position_scale: Vec3::new(40_f64, 32_f64, 40_f64), dimensions: (5_i32, 263_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_cave_layer_71_d5x263x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_cave_layer_71_d5x263x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_cave_layer_69_d5x263x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(4_f64, 4_f64, 4_f64), position_scale: Vec3::new(16_f64, 32_f64, 16_f64), dimensions: (5_i32, 263_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_cave_layer_69_d5x263x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_cave_layer_69_d5x263x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_cave_layer_75_d5x263x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.5_f64, 0.5_f64, 0.5_f64), position_scale: Vec3::new(2_f64, 4_f64, 2_f64), dimensions: (5_i32, 263_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_cave_layer_75_d5x263x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_cave_layer_75_d5x263x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_jagged_72_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1000_f64, 0_f64, 1000_f64), position_scale: Vec3::new(4000_f64, 0_f64, 4000_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_jagged_72_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_jagged_72_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_cave_layer_73_d5x263x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(5_f64, 2_f64, 5_f64), position_scale: Vec3::new(20_f64, 16_f64, 20_f64), dimensions: (5_i32, 263_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_cave_layer_73_d5x263x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_cave_layer_73_d5x263x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_jagged_9_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(500_f64, 0_f64, 500_f64), position_scale: Vec3::new(2000_f64, 0_f64, 2000_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_jagged_9_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_jagged_9_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_jagged_9_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(500_f64, 0_f64, 500_f64), position_scale: Vec3::new(2000_f64, 0_f64, 2000_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_jagged_9_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_jagged_9_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_cave_layer_74_d5x263x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(2.5_f64, 1_f64, 2.5_f64), position_scale: Vec3::new(10_f64, 8_f64, 10_f64), dimensions: (5_i32, 263_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_cave_layer_74_d5x263x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_cave_layer_74_d5x263x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_jagged_18_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(250_f64, 0_f64, 250_f64), position_scale: Vec3::new(1000_f64, 0_f64, 1000_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_jagged_18_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_jagged_18_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_jagged_18_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(250_f64, 0_f64, 250_f64), position_scale: Vec3::new(1000_f64, 0_f64, 1000_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_jagged_18_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_jagged_18_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_ore_vein_a_78_d5x263x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(4_f64, 4_f64, 4_f64), position_scale: Vec3::new(16_f64, 32_f64, 16_f64), dimensions: (5_i32, 263_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_ore_vein_a_78_d5x263x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_ore_vein_a_78_d5x263x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_ore_vein_b_80_d5x263x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(4_f64, 4_f64, 4_f64), position_scale: Vec3::new(16_f64, 32_f64, 16_f64), dimensions: (5_i32, 263_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_ore_vein_b_80_d5x263x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_ore_vein_b_80_d5x263x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_ore_veininess_82_d5x263x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1.5_f64, 1.5_f64, 1.5_f64), position_scale: Vec3::new(6_f64, 12_f64, 6_f64), dimensions: (5_i32, 263_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_ore_veininess_82_d5x263x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_ore_veininess_82_d5x263x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_gravel_70_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.03_f64, 0_f64, 0.03_f64), position_scale: Vec3::new(0.12_f64, 0_f64, 0.12_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_gravel_70_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_gravel_70_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_cave_layer_1_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.2_f64, 0_f64, 0.2_f64), position_scale: Vec3::new(0.8_f64, 0_f64, 0.8_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_cave_layer_1_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_cave_layer_1_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_cave_layer_1_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.2_f64, 0_f64, 0.2_f64), position_scale: Vec3::new(0.8_f64, 0_f64, 0.8_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_cave_layer_1_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_cave_layer_1_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_jagged_5_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_jagged_5_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_jagged_5_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_jagged_5_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_jagged_5_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_jagged_5_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_cave_layer_10_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.15_f64, 0_f64, 0.15_f64), position_scale: Vec3::new(0.6_f64, 0_f64, 0.6_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_cave_layer_10_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_cave_layer_10_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_cave_layer_10_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.15_f64, 0_f64, 0.15_f64), position_scale: Vec3::new(0.6_f64, 0_f64, 0.6_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_cave_layer_10_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_cave_layer_10_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_jagged_21_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(300_f64, 0_f64, 300_f64), position_scale: Vec3::new(1200_f64, 0_f64, 1200_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_jagged_21_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_jagged_21_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_jagged_21_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(300_f64, 0_f64, 300_f64), position_scale: Vec3::new(1200_f64, 0_f64, 1200_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_jagged_21_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_jagged_21_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_gravel_31_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.12_f64, 0_f64, 0.12_f64), position_scale: Vec3::new(0.48_f64, 0_f64, 0.48_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_gravel_31_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_gravel_31_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_gravel_31_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.12_f64, 0_f64, 0.12_f64), position_scale: Vec3::new(0.48_f64, 0_f64, 0.48_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_gravel_31_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_gravel_31_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_cave_layer_7_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.1_f64, 0_f64, 0.1_f64), position_scale: Vec3::new(0.4_f64, 0_f64, 0.4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_cave_layer_7_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_cave_layer_7_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_cave_layer_7_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.1_f64, 0_f64, 0.1_f64), position_scale: Vec3::new(0.4_f64, 0_f64, 0.4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_cave_layer_7_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_cave_layer_7_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_gravel_8_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.1_f64, 0_f64, 0.1_f64), position_scale: Vec3::new(0.4_f64, 0_f64, 0.4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_gravel_8_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_gravel_8_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_gravel_8_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.1_f64, 0_f64, 0.1_f64), position_scale: Vec3::new(0.4_f64, 0_f64, 0.4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_gravel_8_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_gravel_8_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_gravel_2_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.05_f64, 0_f64, 0.05_f64), position_scale: Vec3::new(0.2_f64, 0_f64, 0.2_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_gravel_2_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_gravel_2_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_gravel_2_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.05_f64, 0_f64, 0.05_f64), position_scale: Vec3::new(0.2_f64, 0_f64, 0.2_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_gravel_2_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_gravel_2_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_jagged_3_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(175_f64, 0_f64, 175_f64), position_scale: Vec3::new(700_f64, 0_f64, 700_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_jagged_3_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_jagged_3_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_jagged_3_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(175_f64, 0_f64, 175_f64), position_scale: Vec3::new(700_f64, 0_f64, 700_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_jagged_3_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_jagged_3_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_gravel_19_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.2_f64, 0_f64, 0.2_f64), position_scale: Vec3::new(0.8_f64, 0_f64, 0.8_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_gravel_19_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_gravel_19_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_gravel_19_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.2_f64, 0_f64, 0.2_f64), position_scale: Vec3::new(0.8_f64, 0_f64, 0.8_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_gravel_19_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_gravel_19_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_cave_layer_22_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.3_f64, 0_f64, 0.3_f64), position_scale: Vec3::new(1.2_f64, 0_f64, 1.2_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_cave_layer_22_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_cave_layer_22_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_cave_layer_22_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.3_f64, 0_f64, 0.3_f64), position_scale: Vec3::new(1.2_f64, 0_f64, 1.2_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_cave_layer_22_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_cave_layer_22_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_gravel_23_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.11_f64, 0_f64, 0.11_f64), position_scale: Vec3::new(0.44_f64, 0_f64, 0.44_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_gravel_23_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_gravel_23_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_gravel_23_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.11_f64, 0_f64, 0.11_f64), position_scale: Vec3::new(0.44_f64, 0_f64, 0.44_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_gravel_23_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_gravel_23_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_cave_layer_30_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.25_f64, 0_f64, 0.25_f64), position_scale: Vec3::new(1_f64, 0_f64, 1_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_cave_layer_30_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_cave_layer_30_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_cave_layer_30_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.25_f64, 0_f64, 0.25_f64), position_scale: Vec3::new(1_f64, 0_f64, 1_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_cave_layer_30_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_cave_layer_30_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_jagged_29_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(501_f64, 0_f64, 501_f64), position_scale: Vec3::new(2004_f64, 0_f64, 2004_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_jagged_29_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_jagged_29_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_jagged_29_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(501_f64, 0_f64, 501_f64), position_scale: Vec3::new(2004_f64, 0_f64, 2004_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_jagged_29_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_jagged_29_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_jagged_39_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(2_f64, 0_f64, 2_f64), position_scale: Vec3::new(8_f64, 0_f64, 8_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_jagged_39_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_jagged_39_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_jagged_39_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(2_f64, 0_f64, 2_f64), position_scale: Vec3::new(8_f64, 0_f64, 8_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_jagged_39_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_jagged_39_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_jagged_37_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(600_f64, 0_f64, 600_f64), position_scale: Vec3::new(2400_f64, 0_f64, 2400_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_jagged_37_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_jagged_37_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_jagged_37_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(600_f64, 0_f64, 600_f64), position_scale: Vec3::new(2400_f64, 0_f64, 2400_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_jagged_37_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_jagged_37_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_gravel_11_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.25_f64, 0_f64, 0.25_f64), position_scale: Vec3::new(1_f64, 0_f64, 1_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_gravel_11_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_gravel_11_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_gravel_11_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.25_f64, 0_f64, 0.25_f64), position_scale: Vec3::new(1_f64, 0_f64, 1_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_gravel_11_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_gravel_11_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_RangeChoice_77_d5x263x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 263_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_RangeChoice_77_d5x263x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_RangeChoice_77_d5x263x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_RangeChoice_79_d5x263x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 263_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_RangeChoice_79_d5x263x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_RangeChoice_79_d5x263x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_RangeChoice_81_d5x263x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 263_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_RangeChoice_81_d5x263x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_RangeChoice_81_d5x263x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_determiner_cave_1_d5x263x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 263_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_determiner_cave_1_d5x263x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_determiner_cave_1_d5x263x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_Add_0_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_Add_0_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_Add_0_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_Abs_4_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_Abs_4_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_Abs_4_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_Add_6_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_Add_6_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_Add_6_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_realism_mountains_slope_height_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_realism_mountains_slope_height_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_realism_mountains_slope_height_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_realism_extreme_mountains_slope_height_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_realism_extreme_mountains_slope_height_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_realism_extreme_mountains_slope_height_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_Abs_38_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_Abs_38_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_Abs_38_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_realism_extreme_hills_factor_original_d5x263x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 263_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_realism_extreme_hills_factor_original_d5x263x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_realism_extreme_hills_factor_original_d5x263x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_Multiply_48_d5x263x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 263_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_Multiply_48_d5x263x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_Multiply_48_d5x263x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_Add_83_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_Add_83_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_Add_83_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_Abs_84_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_Abs_84_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_Abs_84_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_Add_85_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_Add_85_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_Add_85_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_realism_mountains_slope_height_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_realism_mountains_slope_height_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_realism_mountains_slope_height_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_realism_extreme_mountains_slope_height_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_realism_extreme_mountains_slope_height_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_realism_extreme_mountains_slope_height_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_Abs_103_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_Abs_103_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_Abs_103_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_realism_extreme_hills_factor_original_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_realism_extreme_hills_factor_original_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_realism_extreme_hills_factor_original_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_Multiply_12_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_Multiply_12_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_Multiply_12_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_realism_extreme_hills_factor_original_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_realism_extreme_hills_factor_original_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_realism_extreme_hills_factor_original_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_Multiply_86_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_Multiply_86_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_Multiply_86_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn vein_ridged_d16x2096x16_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(1_f64, 1_f64, 1_f64), dimensions: (16_i32, 2096_i32, 16_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = vein_ridged_d16x2096x16(p, (&config), buffers, auto_constants, host_consts);
            buffers.vein_ridged_d16x2096x16_output[index(p) as usize] = r;
        }
    }
    
    pub fn vein_toggle_d16x2096x16_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(1_f64, 1_f64, 1_f64), dimensions: (16_i32, 2096_i32, 16_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = vein_toggle_d16x2096x16(p, (&config), buffers, auto_constants, host_consts);
            buffers.vein_toggle_d16x2096x16_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_new_surface_combined_unprocessed_base_d5x263x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 263_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_new_surface_combined_unprocessed_base_d5x263x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_new_surface_combined_unprocessed_base_d5x263x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_realism_hills_slope_height_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_realism_hills_slope_height_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_realism_hills_slope_height_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_realism_extreme_hills_slope_height_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_realism_extreme_hills_slope_height_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_realism_extreme_hills_slope_height_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_realism_hills_slope_height_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_realism_hills_slope_height_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_realism_hills_slope_height_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_realism_extreme_hills_slope_height_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_realism_extreme_hills_slope_height_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_realism_extreme_hills_slope_height_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_new_surface_height_unprocessed_rivered_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_new_surface_height_unprocessed_rivered_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_new_surface_height_unprocessed_rivered_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_new_surface_height_unprocessed_rivered_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_new_surface_height_unprocessed_rivered_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_caves_underlands_combined_d5x263x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 263_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_caves_underlands_combined_d5x263x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_caves_underlands_combined_d5x263x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_new_surface_height_unprocessed_hollowed_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_new_surface_height_unprocessed_hollowed_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_new_surface_height_unprocessed_hollowed_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_new_surface_height_unprocessed_hollowed_d5x1x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (5_i32, 1_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_new_surface_height_unprocessed_hollowed_d5x1x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_new_surface_height_unprocessed_hollowed_d5x1x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_109_d128x32x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 64_f64, 4_f64), dimensions: (128_i32, 32_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_109_d128x32x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_109_d128x32x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_new_surface_combined_processed_d5x263x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 263_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_new_surface_combined_processed_d5x263x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_new_surface_combined_processed_d5x263x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn preliminary_surface_level_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = preliminary_surface_level_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.preliminary_surface_level_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_new_combination_fixed_d5x263x5_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (5_i32, 263_i32, 5_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_new_combination_fixed_d5x263x5(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_new_combination_fixed_d5x263x5_output[index(p) as usize] = r;
        }
    }
    
    pub fn final_density_with_beardify_d16x2096x16_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(1_f64, 1_f64, 1_f64), dimensions: (16_i32, 2096_i32, 16_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = final_density_with_beardify_d16x2096x16(p, (&config), buffers, auto_constants, host_consts);
            buffers.final_density_with_beardify_d16x2096x16_output[index(p) as usize] = r;
        }
    }
    
    pub fn final_density_d16x2096x16_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(1_f64, 1_f64, 1_f64), dimensions: (16_i32, 2096_i32, 16_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = final_density_d16x2096x16(p, (&config), buffers, auto_constants, host_consts);
            buffers.final_density_d16x2096x16_output[index(p) as usize] = r;
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
        pub minecraft_cave_layer_0_octave__8: PerlinNoiseSampler,
        pub minecraft_cave_layer_1_octave__8: PerlinNoiseSampler,
        pub minecraft_gravel_0_octave__8: PerlinNoiseSampler,
        pub minecraft_gravel_1_octave__8: PerlinNoiseSampler,
        pub minecraft_gravel_0_octave__7: PerlinNoiseSampler,
        pub minecraft_gravel_1_octave__7: PerlinNoiseSampler,
        pub minecraft_gravel_0_octave__6: PerlinNoiseSampler,
        pub minecraft_gravel_1_octave__6: PerlinNoiseSampler,
        pub minecraft_gravel_0_octave__5: PerlinNoiseSampler,
        pub minecraft_gravel_1_octave__5: PerlinNoiseSampler,
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
        pub final_density_with_beardify_d16x2096x16_output: [f64; 536576],
        pub final_density_d16x2096x16_output: [f64; 536576],
        pub vein_ridged_d16x2096x16_output: [f64; 536576],
        pub vein_toggle_d16x2096x16_output: [f64; 536576],
        pub preliminary_surface_level_d128x1x128_output: [f64; 16384],
        pub minecraft_new_combination_fixed_d5x263x5_output: [f64; 6575],
        pub density_function_RangeChoice_77_d5x263x5_output: [f64; 6575],
        pub density_function_RangeChoice_79_d5x263x5_output: [f64; 6575],
        pub density_function_RangeChoice_81_d5x263x5_output: [f64; 6575],
        pub density_function_109_d128x32x128_output: [f64; 524288],
        pub minecraft_new_surface_combined_processed_d5x263x5_output: [f64; 6575],
        pub minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output: [f64; 25],
        pub minecraft_cave_layer_66_d5x263x5_output: [f64; 6575],
        pub minecraft_cave_layer_68_d5x263x5_output: [f64; 6575],
        pub minecraft_cave_layer_71_d5x263x5_output: [f64; 6575],
        pub minecraft_cave_layer_69_d5x263x5_output: [f64; 6575],
        pub minecraft_caves_underlands_combined_d5x263x5_output: [f64; 6575],
        pub minecraft_determiner_cave_1_d5x263x5_output: [f64; 6575],
        pub minecraft_cave_layer_75_d5x263x5_output: [f64; 6575],
        pub minecraft_new_surface_combined_unprocessed_base_d5x263x5_output: [f64; 6575],
        pub minecraft_jagged_72_d5x1x5_output: [f64; 25],
        pub minecraft_cave_layer_73_d5x263x5_output: [f64; 6575],
        pub minecraft_jagged_9_d5x1x5_output: [f64; 25],
        pub minecraft_cave_layer_74_d5x263x5_output: [f64; 6575],
        pub minecraft_jagged_18_d5x1x5_output: [f64; 25],
        pub minecraft_ore_vein_a_78_d5x263x5_output: [f64; 6575],
        pub minecraft_ore_vein_b_80_d5x263x5_output: [f64; 6575],
        pub minecraft_ore_veininess_82_d5x263x5_output: [f64; 6575],
        pub minecraft_new_surface_height_unprocessed_hollowed_d128x1x128_output: [f64; 16384],
        pub minecraft_new_surface_height_unprocessed_hollowed_d5x1x5_output: [f64; 25],
        pub density_function_Add_0_d5x1x5_output: [f64; 25],
        pub density_function_Abs_4_d5x1x5_output: [f64; 25],
        pub density_function_Add_6_d5x1x5_output: [f64; 25],
        pub minecraft_realism_hills_slope_height_d5x1x5_output: [f64; 25],
        pub minecraft_realism_extreme_hills_slope_height_d5x1x5_output: [f64; 25],
        pub minecraft_realism_mountains_slope_height_d5x1x5_output: [f64; 25],
        pub minecraft_realism_extreme_mountains_slope_height_d5x1x5_output: [f64; 25],
        pub density_function_Abs_38_d5x1x5_output: [f64; 25],
        pub minecraft_gravel_70_d5x1x5_output: [f64; 25],
        pub minecraft_cave_layer_1_d5x1x5_output: [f64; 25],
        pub minecraft_jagged_5_d5x1x5_output: [f64; 25],
        pub minecraft_cave_layer_10_d5x1x5_output: [f64; 25],
        pub minecraft_jagged_21_d5x1x5_output: [f64; 25],
        pub minecraft_gravel_31_d5x1x5_output: [f64; 25],
        pub minecraft_cave_layer_7_d5x1x5_output: [f64; 25],
        pub minecraft_gravel_8_d5x1x5_output: [f64; 25],
        pub minecraft_gravel_2_d5x1x5_output: [f64; 25],
        pub minecraft_jagged_3_d5x1x5_output: [f64; 25],
        pub minecraft_realism_extreme_hills_factor_original_d5x263x5_output: [f64; 6575],
        pub density_function_Multiply_48_d5x263x5_output: [f64; 6575],
        pub minecraft_gravel_19_d5x1x5_output: [f64; 25],
        pub minecraft_cave_layer_22_d5x1x5_output: [f64; 25],
        pub minecraft_gravel_23_d5x1x5_output: [f64; 25],
        pub minecraft_cave_layer_30_d5x1x5_output: [f64; 25],
        pub minecraft_jagged_29_d5x1x5_output: [f64; 25],
        pub minecraft_jagged_39_d5x1x5_output: [f64; 25],
        pub minecraft_jagged_37_d5x1x5_output: [f64; 25],
        pub minecraft_new_surface_height_unprocessed_rivered_d128x1x128_output: [f64; 16384],
        pub density_function_Add_83_d128x1x128_output: [f64; 16384],
        pub density_function_Abs_84_d128x1x128_output: [f64; 16384],
        pub density_function_Add_85_d128x1x128_output: [f64; 16384],
        pub minecraft_realism_hills_slope_height_d128x1x128_output: [f64; 16384],
        pub minecraft_realism_extreme_hills_slope_height_d128x1x128_output: [f64; 16384],
        pub minecraft_realism_mountains_slope_height_d128x1x128_output: [f64; 16384],
        pub minecraft_realism_extreme_mountains_slope_height_d128x1x128_output: [f64; 16384],
        pub density_function_Abs_103_d128x1x128_output: [f64; 16384],
        pub minecraft_realism_extreme_hills_factor_original_d5x1x5_output: [f64; 25],
        pub density_function_Multiply_12_d5x1x5_output: [f64; 25],
        pub minecraft_gravel_11_d5x1x5_output: [f64; 25],
        pub minecraft_cave_layer_1_d128x1x128_output: [f64; 16384],
        pub minecraft_gravel_2_d128x1x128_output: [f64; 16384],
        pub minecraft_jagged_3_d128x1x128_output: [f64; 16384],
        pub minecraft_jagged_5_d128x1x128_output: [f64; 16384],
        pub minecraft_cave_layer_7_d128x1x128_output: [f64; 16384],
        pub minecraft_gravel_8_d128x1x128_output: [f64; 16384],
        pub minecraft_jagged_9_d128x1x128_output: [f64; 16384],
        pub minecraft_cave_layer_10_d128x1x128_output: [f64; 16384],
        pub minecraft_realism_extreme_hills_factor_original_d128x1x128_output: [f64; 16384],
        pub density_function_Multiply_86_d128x1x128_output: [f64; 16384],
        pub minecraft_gravel_19_d128x1x128_output: [f64; 16384],
        pub minecraft_jagged_18_d128x1x128_output: [f64; 16384],
        pub minecraft_jagged_21_d128x1x128_output: [f64; 16384],
        pub minecraft_cave_layer_22_d128x1x128_output: [f64; 16384],
        pub minecraft_gravel_23_d128x1x128_output: [f64; 16384],
        pub minecraft_jagged_29_d128x1x128_output: [f64; 16384],
        pub minecraft_cave_layer_30_d128x1x128_output: [f64; 16384],
        pub minecraft_gravel_31_d128x1x128_output: [f64; 16384],
        pub minecraft_jagged_37_d128x1x128_output: [f64; 16384],
        pub minecraft_jagged_39_d128x1x128_output: [f64; 16384],
        pub minecraft_gravel_11_d128x1x128_output: [f64; 16384],
    }
    
    #[derive(Clone)]
    pub struct Orchestration {
        pub buffers: Box<InternalBuffers>,
        pub auto_constants: Box<AutoConsts>,
    }
    
    fn make_auto_consts(seed: i64, auto_consts: &mut AutoConsts) {
        auto_consts.minecraft_cave_layer_0_octave__8 = make_permutation_table(seed, 5620043451534219907_i64, 6506268541312725510_i64, 0_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_cave_layer_1_octave__8 = make_permutation_table(seed, 5620043451534219907_i64, 6506268541312725510_i64, 1_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_gravel_0_octave__8 = make_permutation_table(seed, 1153479925840222391_i64, 7756889976470315235_i64, 0_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_gravel_1_octave__8 = make_permutation_table(seed, 1153479925840222391_i64, 7756889976470315235_i64, 1_i64, 1078206144088113246_i64, 5239585857299060288_i64);
        auto_consts.minecraft_gravel_0_octave__7 = make_permutation_table(seed, 1153479925840222391_i64, 7756889976470315235_i64, 0_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_gravel_1_octave__7 = make_permutation_table(seed, 1153479925840222391_i64, 7756889976470315235_i64, 1_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_gravel_0_octave__6 = make_permutation_table(seed, 1153479925840222391_i64, 7756889976470315235_i64, 0_i64, -1937505727046359452_i64, 6887262389667105861_i64);
        auto_consts.minecraft_gravel_1_octave__6 = make_permutation_table(seed, 1153479925840222391_i64, 7756889976470315235_i64, 1_i64, -1937505727046359452_i64, 6887262389667105861_i64);
        auto_consts.minecraft_gravel_0_octave__5 = make_permutation_table(seed, 1153479925840222391_i64, 7756889976470315235_i64, 0_i64, 7888980432583755018_i64, 3328269827262531447_i64);
        auto_consts.minecraft_gravel_1_octave__5 = make_permutation_table(seed, 1153479925840222391_i64, 7756889976470315235_i64, 1_i64, 7888980432583755018_i64, 3328269827262531447_i64);
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
        
        pub fn orchestrate_final_density_with_beardify(&mut self, origin: Vec3, host_consts: &HostConsts) -> &[f64; 536576] {
            let _kernel_call_0_0: () = minecraft_cave_layer_66_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_1: () = minecraft_cave_layer_68_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_2: () = minecraft_cave_layer_71_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_3: () = minecraft_cave_layer_69_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_4: () = minecraft_cave_layer_75_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_5: () = minecraft_jagged_72_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_6: () = minecraft_cave_layer_73_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_7: () = minecraft_jagged_9_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_8: () = minecraft_cave_layer_74_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_9: () = minecraft_jagged_18_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_10: () = minecraft_gravel_70_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_11: () = minecraft_cave_layer_1_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_12: () = minecraft_jagged_5_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_13: () = minecraft_cave_layer_10_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_14: () = minecraft_jagged_21_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_15: () = minecraft_gravel_31_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_16: () = minecraft_cave_layer_7_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_17: () = minecraft_gravel_8_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_18: () = minecraft_gravel_2_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_19: () = minecraft_jagged_3_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_20: () = minecraft_gravel_19_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_21: () = minecraft_cave_layer_22_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_22: () = minecraft_gravel_23_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_23: () = minecraft_cave_layer_30_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_24: () = minecraft_jagged_29_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_25: () = minecraft_jagged_39_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_26: () = minecraft_jagged_37_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_27: () = minecraft_gravel_11_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_0: () = minecraft_determiner_cave_1_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_1: () = density_function_Add_0_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_2: () = density_function_Abs_4_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_3: () = density_function_Add_6_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_4: () = minecraft_realism_mountains_slope_height_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_5: () = minecraft_realism_extreme_mountains_slope_height_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_6: () = density_function_Abs_38_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_7: () = minecraft_realism_extreme_hills_factor_original_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_8: () = density_function_Multiply_48_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_9: () = minecraft_realism_extreme_hills_factor_original_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_10: () = density_function_Multiply_12_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_0: () = minecraft_new_surface_combined_unprocessed_base_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_1: () = minecraft_realism_hills_slope_height_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_2: () = minecraft_realism_extreme_hills_slope_height_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_3_0: () = minecraft_new_surface_height_unprocessed_rivered_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_4_0: () = minecraft_caves_underlands_combined_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_4_1: () = minecraft_new_surface_height_unprocessed_hollowed_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_5_0: () = minecraft_new_surface_combined_processed_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_6_0: () = minecraft_new_combination_fixed_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_7_0: () = final_density_with_beardify_d16x2096x16_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            return (&self.buffers.final_density_with_beardify_d16x2096x16_output);
        }
        
        pub fn orchestrate_final_density(&mut self, origin: Vec3, host_consts: &HostConsts) -> &[f64; 536576] {
            let _kernel_call_0_0: () = minecraft_cave_layer_66_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_1: () = minecraft_cave_layer_68_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_2: () = minecraft_cave_layer_71_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_3: () = minecraft_cave_layer_69_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_4: () = minecraft_cave_layer_75_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_5: () = minecraft_jagged_72_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_6: () = minecraft_cave_layer_73_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_7: () = minecraft_jagged_9_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_8: () = minecraft_cave_layer_74_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_9: () = minecraft_jagged_18_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_10: () = minecraft_gravel_70_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_11: () = minecraft_cave_layer_1_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_12: () = minecraft_jagged_5_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_13: () = minecraft_cave_layer_10_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_14: () = minecraft_jagged_21_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_15: () = minecraft_gravel_31_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_16: () = minecraft_cave_layer_7_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_17: () = minecraft_gravel_8_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_18: () = minecraft_gravel_2_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_19: () = minecraft_jagged_3_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_20: () = minecraft_gravel_19_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_21: () = minecraft_cave_layer_22_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_22: () = minecraft_gravel_23_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_23: () = minecraft_cave_layer_30_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_24: () = minecraft_jagged_29_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_25: () = minecraft_jagged_39_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_26: () = minecraft_jagged_37_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_27: () = minecraft_gravel_11_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_0: () = minecraft_determiner_cave_1_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_1: () = density_function_Add_0_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_2: () = density_function_Abs_4_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_3: () = density_function_Add_6_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_4: () = minecraft_realism_mountains_slope_height_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_5: () = minecraft_realism_extreme_mountains_slope_height_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_6: () = density_function_Abs_38_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_7: () = minecraft_realism_extreme_hills_factor_original_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_8: () = density_function_Multiply_48_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_9: () = minecraft_realism_extreme_hills_factor_original_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_10: () = density_function_Multiply_12_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_0: () = minecraft_new_surface_combined_unprocessed_base_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_1: () = minecraft_realism_hills_slope_height_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_2: () = minecraft_realism_extreme_hills_slope_height_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_3_0: () = minecraft_new_surface_height_unprocessed_rivered_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_4_0: () = minecraft_caves_underlands_combined_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_4_1: () = minecraft_new_surface_height_unprocessed_hollowed_d5x1x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_5_0: () = minecraft_new_surface_combined_processed_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_6_0: () = minecraft_new_combination_fixed_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_7_0: () = final_density_d16x2096x16_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            return (&self.buffers.final_density_d16x2096x16_output);
        }
        
        pub fn orchestrate_vein_ridged(&mut self, origin: Vec3, host_consts: &HostConsts) -> &[f64; 536576] {
            let _kernel_call_0_0: () = minecraft_ore_vein_a_78_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_1: () = minecraft_ore_vein_b_80_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_0: () = density_function_RangeChoice_77_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_1: () = density_function_RangeChoice_79_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_0: () = vein_ridged_d16x2096x16_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            return (&self.buffers.vein_ridged_d16x2096x16_output);
        }
        
        pub fn orchestrate_vein_toggle(&mut self, origin: Vec3, host_consts: &HostConsts) -> &[f64; 536576] {
            let _kernel_call_0_0: () = minecraft_ore_veininess_82_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_0: () = density_function_RangeChoice_81_d5x263x5_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_0: () = vein_toggle_d16x2096x16_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            return (&self.buffers.vein_toggle_d16x2096x16_output);
        }
        
        pub fn orchestrate_preliminary_surface_level(&mut self, origin: Vec3, host_consts: &HostConsts) -> &[f64; 16384] {
            let _kernel_call_0_0: () = minecraft_cave_layer_1_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_1: () = minecraft_gravel_2_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_2: () = minecraft_jagged_3_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_3: () = minecraft_jagged_5_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_4: () = minecraft_cave_layer_7_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_5: () = minecraft_gravel_8_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_6: () = minecraft_jagged_9_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_7: () = minecraft_cave_layer_10_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_8: () = minecraft_gravel_19_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_9: () = minecraft_jagged_18_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_10: () = minecraft_jagged_21_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_11: () = minecraft_cave_layer_22_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_12: () = minecraft_gravel_23_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_13: () = minecraft_jagged_29_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_14: () = minecraft_cave_layer_30_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_15: () = minecraft_gravel_31_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_16: () = minecraft_jagged_37_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_17: () = minecraft_jagged_39_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_18: () = minecraft_gravel_11_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_0: () = density_function_Add_83_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_1: () = density_function_Abs_84_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_2: () = density_function_Add_85_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_3: () = minecraft_realism_mountains_slope_height_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_4: () = minecraft_realism_extreme_mountains_slope_height_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_5: () = density_function_Abs_103_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_6: () = minecraft_realism_extreme_hills_factor_original_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_7: () = density_function_Multiply_86_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_0: () = minecraft_realism_hills_slope_height_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_1: () = minecraft_realism_extreme_hills_slope_height_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_3_0: () = minecraft_new_surface_height_unprocessed_rivered_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_4_0: () = minecraft_new_surface_height_unprocessed_hollowed_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_5_0: () = density_function_109_d128x32x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_6_0: () = preliminary_surface_level_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            return (&self.buffers.preliminary_surface_level_d128x1x128_output);
        }
    }
    
}
