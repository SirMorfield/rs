#include "computer.hpp"
#include "env.hpp"
#include <iostream>
#include <unistd.h>

extern "C" int mainj(int argc, char *argv[])
{
	(void)argc;
	(void)argv;

	std::string path;
	if (argc == 2)
		path = argv[1];
	else if (argc == 1)
		path = "rt_test/ball.rt";
	else
	{
		std::cout << "Usage: " << argv[0] << "[.rt file]" << std::endl;
		exit(1);
	}

	env::print();

	Time scene_parse_time("Parsed scene in");
	Scene scene(path);
	scene_parse_time.print();
	std::cout << scene << std::endl;

	const Resolution resolution = scene.resolution;
	std::cout << resolution << std::endl;

	Frame_buffer fb(resolution, env::log_progress);
	Renderer renderer(resolution);
	std::vector<std::thread> threads(env::threads);

	for (size_t i = 0; i < threads.capacity(); i++)
		threads[i] = std::thread(&Renderer::thread, &renderer, scene, &fb);

	Time render_time("Render time");
	for (auto &th : threads)
		th.join();
	render_time.print();

	fb.save_to_BMP();
	return 0;
}
