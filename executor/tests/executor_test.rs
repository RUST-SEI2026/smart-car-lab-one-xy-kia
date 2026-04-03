use executor::{Executor, Pose};

//移动测试
mod move_tests{
    use super::*;

    #[test]
    fn should_return_x_plus_1_given_command_is_m_and_facing_is_e(){
        //测试e方向下m指令
        let original_pose = Pose::new(0,0,'E');
        //初始位置
        let mut executor = Executor::with_pose(original_pose);
        executor.execute("M");
        //移动
        let expected_pose = Pose::new(1,0,'E');
        //预测位置
        assert_eq!(expected_pose, executor.query());
        //判断
    }

    #[test]
    fn should_return_x_minus_1_given_command_is_m_and_facing_is_w(){
        //测试w方向下m指令
        let original_pose = Pose::new(0,0,'W');
        //初始位置
        let mut executor = Executor::with_pose(original_pose);
        executor.execute("M");
        //移动
        let expected_pose = Pose::new(-1,0,'W');
        //预测位置
        assert_eq!(expected_pose, executor.query());
        //判断
    }

    #[test]
    fn should_return_y_plus_1_given_command_is_m_and_facing_is_n(){
        //测试n方向下m指令
        let original_pose = Pose::new(0,0,'N');
        //初始位置
        let mut executor = Executor::with_pose(original_pose);
        executor.execute("M");
        //移动
        let expected_pose = Pose::new(0,1,'N');
        //预测位置
        assert_eq!(expected_pose, executor.query());
        //判断
    }

    #[test]
    fn should_return_y_minus_1_given_command_is_m_and_facing_is_s(){
        //测试s方向下m指令
        let original_pose = Pose::new(0,0,'S');
        //初始位置
        let mut executor = Executor::with_pose(original_pose);
        executor.execute("M");
        //移动
        let expected_pose = Pose::new(0,-1,'S');
        //预测位置
        assert_eq!(expected_pose, executor.query());
        //判断
    }

    #[test]
    fn should_return_heading_n_given_command_is_l_and_facing_is_e(){
        //测试e方向下l指令
        let original_pose = Pose::new(0,0,'E');
        //初始位置
        let mut executor = Executor::with_pose(original_pose);
        executor.execute("L");
        //转向
        let expected_pose = Pose::new(0,0,'N');
        //预测位置
        assert_eq!(expected_pose, executor.query());
        //判断
    }

    #[test]
    fn should_return_heading_e_given_command_is_l_and_facing_is_s(){
        //测试s方向下l指令
        let original_pose = Pose::new(0,0,'S');
        //初始位置
        let mut executor = Executor::with_pose(original_pose);
        executor.execute("L");
        //转向
        let expected_pose = Pose::new(0,0,'E');
        //预测位置
        assert_eq!(expected_pose, executor.query());
        //判断
    }

    #[test]
    fn should_return_heading_s_given_command_is_l_and_facing_is_w(){
        //测试w方向下l指令
        let original_pose = Pose::new(0,0,'W');
        //初始位置
        let mut executor = Executor::with_pose(original_pose);
        executor.execute("L");
        //转向
        let expected_pose = Pose::new(0,0,'S');
        //预测位置
        assert_eq!(expected_pose, executor.query());
        //判断
    }

    #[test]
    fn should_return_heading_w_given_command_is_l_and_facing_is_n(){
        //测试n方向下l指令
        let original_pose = Pose::new(0,0,'N');
        //初始位置
        let mut executor = Executor::with_pose(original_pose);
        executor.execute("L");
        //转向
        let expected_pose = Pose::new(0,0,'W');
        //预测位置
        assert_eq!(expected_pose, executor.query());
        //判断
    }
}