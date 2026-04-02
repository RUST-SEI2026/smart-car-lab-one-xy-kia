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
}