import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:scrum_deck/app/screens/sprint/sprint_block.dart';
import 'package:scrum_deck/app/screens/sprint/sprint_module.dart';
import 'package:scrum_deck/shared/models/sprint.dart';

class SprintWidget extends StatelessWidget {
  late final SprintBloc _bloc = SprintModule.to.getBloc<SprintBloc>();

  void _pushCreate(context) {
    Navigator.of(context).push(
      MaterialPageRoute<void>(
        builder: (context) {
          return Scaffold(
            appBar: AppBar(
              title: const Text('Criando sprint'),
            ),
            body: Container(
              margin: new EdgeInsets.symmetric(horizontal: 20.0),
              child: Form(
                key: _bloc.formKey,
                child: Column(
                  children: <Widget>[
                    SizedBox(height: 50),
                    Align(
                        alignment: Alignment.centerLeft,
                        child: Text("Nome da sprint")),
                    TextFormField(
                      controller: _bloc.nameController,
                      decoration: const InputDecoration(
                        hintText: 'Sprint de numero 45',
                      ),
                      validator: (value) {
                        if (value != null && value.isEmpty) {
                          return 'O nome da sprint deve ser prenchido.';
                        }
                        return null;
                      },
                    ),
                    SizedBox(height: 50),
                    Align(
                        alignment: Alignment.centerLeft,
                        child: Text("Link para sprint")),
                    TextFormField(
                      controller: _bloc.linkController,
                      decoration: const InputDecoration(
                        hintText: 'https://trello.com/board/1d2398dj',
                      ),

                      validator: (value) {
                        if (value != null && value.isEmpty) {
                          return 'O link da sprint deve ser prenchido.';
                        }
                        bool validLink = RegExp(r"https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)").hasMatch(value!);
                        if(!validLink){
                          return "Link invalido";
                        }
                        return null;
                      },
                    ),
                    SizedBox(height: 10),
                    ElevatedButton(
                      onPressed: () {
                        if (_bloc.formKey.currentState!.validate()) {
                          ScaffoldMessenger.of(context).showSnackBar(
                              SnackBar(content: Text('Salvando sprint')));
                          _bloc.doSave();
                          Navigator.of(context).pop();
                        }
                      },
                      child: Text('Salvar nova sprint'),
                    ),
                  ],
                ),
              ),
            ),
          );
        },
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    _bloc.doFetch();

    return Scaffold(
      appBar: AppBar(
        title: Text('Sprints'),
      ),
      body: buildSprintList(),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          _pushCreate(context);
        },
        child: const Icon(Icons.add),
        backgroundColor: Colors.blue,
      ),
    );
  }

  StreamBuilder<List<Sprint>> buildSprintList() {
    return StreamBuilder(
      stream: _bloc.sprints,
      builder: (_, AsyncSnapshot<List<Sprint>> snapshot) {
        if (snapshot.hasData) {
          return ListView.separated(
              itemBuilder: (_, index) {
                final post = snapshot.data![index];
                return ListTile(
                  title: Text(post.nome),
                  subtitle: Text(post.link),
                  trailing: IconButton(
                    icon: Icon(
                      Icons.delete,
                      color: Colors.grey,
                    ),
                    onPressed: () {
                      _bloc.doDelete(post.id!);
                    },
                  ),
                );
              },
              separatorBuilder: (_, __) => Divider(),
              itemCount: snapshot.data!.length);
        } else {
          return StreamBuilder(
            stream: _bloc.loading,
            builder: (_, AsyncSnapshot snapshot) {
              final loading = snapshot.data ?? false;
              if (loading) {
                return Center(
                  child: CircularProgressIndicator(),
                );
              }
              return Container();
            },
          );
        }
      },
    );
  }
}
