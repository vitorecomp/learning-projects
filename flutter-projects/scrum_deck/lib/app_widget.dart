import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:scrum_deck/app/screens/sprint/sprint_module.dart';
import 'package:scrum_deck/app/screens/sprint/sprint_widget.dart';

class AppWidget extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData.dark(),
      home: SprintModule(),
    );
  }
}