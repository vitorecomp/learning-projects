import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:scrum_deck/app/screens/sprint/sprint_block.dart';
import 'package:scrum_deck/app/screens/sprint/sprint_module.dart';
import 'package:scrum_deck/shared/models/sprint.dart';

class SprintWidget extends StatelessWidget {

  late final SprintBloc _bloc = SprintModule.to.getBloc<SprintBloc>();

  @override
  Widget build(BuildContext context) {
    _bloc.doFetch();

    return Scaffold(
      appBar: AppBar(
        title: Text('Sprints'),
      ),
      body: StreamBuilder(
        stream: _bloc.sprints,
        builder: (_, AsyncSnapshot<List<Sprint>> snapshot){
           if(snapshot.hasData){
             return ListView.separated(
                 itemBuilder: (_, index){
                   final post = snapshot.data![index];
                   return ListTile(
                     title: Text(post.nome),
                      subtitle: Text(post.link),
                   );
                 },
                 separatorBuilder: (_, __) => Divider(),
                 itemCount: snapshot.data!.length
             );
           }else{
             return StreamBuilder(
                 stream: _bloc.loading,
                 builder: (_, AsyncSnapshot snapshot){
                   final loading = snapshot.data ?? false;
                   if(loading){
                     return Center(child: CircularProgressIndicator(),);
                   }
                     return Container();
                 },
              );
           }
        },
      )
    );
  }

}