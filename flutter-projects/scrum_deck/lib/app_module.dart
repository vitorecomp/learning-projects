import 'package:bloc_pattern/bloc_pattern.dart';
import 'package:flutter/src/widgets/framework.dart';
import 'package:http/http.dart';
import 'package:http/retry.dart';
import 'package:scrum_deck/app_block.dart';
import 'package:scrum_deck/app_widget.dart';

class AppModule extends ModuleWidget {
  @override
  List<Bloc> get blocs => [
    Bloc((i) => AppBloc())
  ];

  @override
  List<Dependency> get dependencies => [
    Dependency((i) => RetryClient(Client()))
  ];

  @override
  Widget get view => AppWidget();

  static Inject get to => Inject<AppModule>.of();

}