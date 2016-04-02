var searchIndex = {};
searchIndex["minwin"] = {"doc":"","items":[[0,"access","minwin","",null,null],[3,"CustomAccess","minwin::access","",null,null],[12,"0","","",0,null],[3,"MaximumAccess","","",null,null],[3,"SystemSecurityAccess","","",null,null],[4,"GenericAccess","","",null,null],[13,"All","","",1,null],[13,"Read","","",1,null],[13,"Write","","",1,null],[13,"Execute","","",1,null],[4,"StandardAccess","","",null,null],[13,"Delete","","",2,null],[13,"ReadControl","","",2,null],[13,"WriteDac","","",2,null],[13,"WriteOwner","","",2,null],[13,"Synchronize","","",2,null],[8,"Access","","",null,null],[10,"mask","","",3,{"inputs":[{"name":"access"}],"output":{"name":"u32"}}],[8,"CombinableAccess","","",null,null],[11,"combine","","",4,{"inputs":[{"name":"combinableaccess"},{"name":"t"}],"output":{"name":"customaccess"}}],[11,"mask","","",0,{"inputs":[{"name":"customaccess"}],"output":{"name":"u32"}}],[11,"bitor","","",0,{"inputs":[{"name":"customaccess"},{"name":"t"}],"output":{"name":"customaccess"}}],[11,"mask","","",5,{"inputs":[{"name":"maximumaccess"}],"output":{"name":"u32"}}],[11,"mask","","",6,{"inputs":[{"name":"systemsecurityaccess"}],"output":{"name":"u32"}}],[11,"bitor","","",6,{"inputs":[{"name":"systemsecurityaccess"},{"name":"t"}],"output":{"name":"customaccess"}}],[11,"eq","","",1,{"inputs":[{"name":"genericaccess"},{"name":"genericaccess"}],"output":{"name":"bool"}}],[11,"clone","","",1,{"inputs":[{"name":"genericaccess"}],"output":{"name":"genericaccess"}}],[11,"fmt","","",1,{"inputs":[{"name":"genericaccess"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"mask","","",1,{"inputs":[{"name":"genericaccess"}],"output":{"name":"u32"}}],[11,"bitor","","",1,{"inputs":[{"name":"genericaccess"},{"name":"t"}],"output":{"name":"customaccess"}}],[11,"eq","","",2,{"inputs":[{"name":"standardaccess"},{"name":"standardaccess"}],"output":{"name":"bool"}}],[11,"clone","","",2,{"inputs":[{"name":"standardaccess"}],"output":{"name":"standardaccess"}}],[11,"fmt","","",2,{"inputs":[{"name":"standardaccess"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"mask","","",2,{"inputs":[{"name":"standardaccess"}],"output":{"name":"u32"}}],[11,"all","","",2,{"inputs":[],"output":{"name":"customaccess"}}],[11,"read","","",2,{"inputs":[],"output":{"name":"customaccess"}}],[11,"write","","",2,{"inputs":[],"output":{"name":"customaccess"}}],[11,"execute","","",2,{"inputs":[],"output":{"name":"customaccess"}}],[11,"required","","",2,{"inputs":[],"output":{"name":"customaccess"}}],[11,"bitor","","",2,{"inputs":[{"name":"standardaccess"},{"name":"t"}],"output":{"name":"customaccess"}}],[0,"handle","minwin","",null,null],[3,"Handle","minwin::handle","",null,null],[5,"duplicate","","",null,{"inputs":[{"name":"rawhandle"}],"output":{"name":"result"}}],[5,"set_inheritable","","",null,{"inputs":[{"name":"rawhandle"},{"name":"bool"}],"output":{"name":"result"}}],[5,"is_inheritable","","",null,{"inputs":[{"name":"rawhandle"}],"output":{"name":"result"}}],[5,"set_protected","","",null,{"inputs":[{"name":"rawhandle"},{"name":"bool"}],"output":{"name":"result"}}],[5,"is_protected","","",null,{"inputs":[{"name":"rawhandle"}],"output":{"name":"result"}}],[11,"fmt","","",7,{"inputs":[{"name":"handle"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"drop","","",7,{"inputs":[{"name":"handle"}],"output":null}],[11,"from_raw_handle","","",7,{"inputs":[{"name":"rawhandle"}],"output":{"name":"handle"}}],[11,"as_raw_handle","","",7,{"inputs":[{"name":"handle"}],"output":{"name":"rawhandle"}}],[11,"into_raw_handle","","",7,{"inputs":[{"name":"handle"}],"output":{"name":"rawhandle"}}],[0,"named","minwin","",null,null],[3,"NamedOpenOptions","minwin::named","Options for opening a named object.",null,null],[4,"CreateNamedError","","",null,null],[13,"AlreadyExists","","",8,null],[13,"InvalidName","","",8,null],[13,"Io","","",8,null],[6,"CreateNamedResult","","",null,null],[6,"NamedOpenFunction","","Windows API function type used for opening named objects.",null,null],[8,"NamedBuilder","","A trait containing common methods for creating named objects.",null,null],[16,"Output","","",9,null],[11,"create","","Creates a new anonymous object.",9,{"inputs":[{"name":"namedbuilder"}],"output":{"name":"result"}}],[11,"create_named","","Creates a new named object or opens an existing object.",9,{"inputs":[{"name":"namedbuilder"},{"name":"n"}],"output":{"name":"createnamedresult"}}],[8,"NamedObject","","A type that can be constructed given an existing object name.",null,null],[10,"open_function","","The extern function used for opening an object of this type.",10,{"inputs":[],"output":{"name":"namedopenfunction"}}],[10,"default_open_options","","The default opening options for this type.",10,{"inputs":[],"output":{"name":"namedopenoptions"}}],[11,"open","","Opens a named object using default options.",10,{"inputs":[{"name":"n"}],"output":{"name":"result"}}],[11,"open_with_options","","Opens a named object using the specified options.",10,{"inputs":[{"name":"n"},{"name":"o"}],"output":{"name":"result"}}],[11,"fmt","","",8,{"inputs":[{"name":"createnamederror"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",8,{"inputs":[{"name":"createnamederror"}],"output":{"name":"str"}}],[11,"cause","","",8,{"inputs":[{"name":"createnamederror"}],"output":{"name":"option"}}],[11,"fmt","","",8,{"inputs":[{"name":"createnamederror"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from","","",8,{"inputs":[{"name":"error"}],"output":{"name":"createnamederror"}}],[11,"from","","",8,{"inputs":[{"name":"nulerror"}],"output":{"name":"createnamederror"}}],[11,"unwrap","","",8,{"inputs":[{"name":"createnamederror"}],"output":{"name":"option"}}],[11,"fmt","","",11,{"inputs":[{"name":"namedopenoptions"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","",11,{"inputs":[],"output":{"name":"namedopenoptions"}}],[11,"inheritable","","Indicates whether the underlying handle can be inherited.",11,{"inputs":[{"name":"namedopenoptions"},{"name":"bool"}],"output":{"name":"self"}}],[11,"desired_access","","The desired access for the object.",11,{"inputs":[{"name":"namedopenoptions"},{"name":"a"}],"output":{"name":"self"}}],[11,"from","","",11,{"inputs":[{"name":"a"}],"output":{"name":"namedopenoptions"}}],[0,"mapping","minwin","",null,null],[3,"FileMappingBuilder","minwin::mapping","",null,null],[3,"FileMapping","","",null,null],[3,"FileViewBuilder","","",null,null],[3,"FileView","","",null,null],[4,"FileMappingAttribute","","",null,null],[13,"Commit","","",12,null],[13,"Image","","",12,null],[13,"ImageNoExecute","","",12,null],[13,"LargePages","","",12,null],[13,"NoCache","","",12,null],[13,"Reserve","","",12,null],[13,"WriteCombine","","",12,null],[4,"FileMappingAccess","","",null,null],[13,"Read","","",13,null],[13,"Write","","",13,null],[13,"Execute","","",13,null],[4,"FileViewWriteMode","","",null,null],[13,"ReadOnly","","",14,null],[13,"Write","","",14,null],[13,"CopyOnWrite","","",14,null],[11,"eq","","",12,{"inputs":[{"name":"filemappingattribute"},{"name":"filemappingattribute"}],"output":{"name":"bool"}}],[11,"clone","","",12,{"inputs":[{"name":"filemappingattribute"}],"output":{"name":"filemappingattribute"}}],[11,"fmt","","",12,{"inputs":[{"name":"filemappingattribute"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","",15,{"inputs":[{"name":"u64"}],"output":{"name":"filemappingbuilder"}}],[11,"from_file","","",15,{"inputs":[{"name":"f"}],"output":{"name":"filemappingbuilder"}}],[11,"size","","",15,{"inputs":[{"name":"filemappingbuilder"},{"name":"u64"}],"output":{"name":"filemappingbuilder"}}],[11,"writable","","",15,{"inputs":[{"name":"filemappingbuilder"},{"name":"bool"}],"output":{"name":"filemappingbuilder"}}],[11,"executable","","",15,{"inputs":[{"name":"filemappingbuilder"},{"name":"bool"}],"output":{"name":"filemappingbuilder"}}],[11,"attribute","","",15,{"inputs":[{"name":"filemappingbuilder"},{"name":"filemappingattribute"}],"output":{"name":"filemappingbuilder"}}],[11,"attributes","","",15,null],[11,"name","","",15,{"inputs":[{"name":"filemappingbuilder"},{"name":"n"}],"output":{"name":"filemappingbuilder"}}],[11,"fmt","","",16,{"inputs":[{"name":"filemapping"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from_raw_handle","","",16,{"inputs":[{"name":"rawhandle"}],"output":{"name":"filemapping"}}],[11,"as_raw_handle","","",16,{"inputs":[{"name":"filemapping"}],"output":{"name":"rawhandle"}}],[11,"into_raw_handle","","",16,{"inputs":[{"name":"filemapping"}],"output":{"name":"rawhandle"}}],[11,"eq","","",13,{"inputs":[{"name":"filemappingaccess"},{"name":"filemappingaccess"}],"output":{"name":"bool"}}],[11,"clone","","",13,{"inputs":[{"name":"filemappingaccess"}],"output":{"name":"filemappingaccess"}}],[11,"fmt","","",13,{"inputs":[{"name":"filemappingaccess"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"mask","","",13,{"inputs":[{"name":"filemappingaccess"}],"output":{"name":"u32"}}],[11,"all","","",13,{"inputs":[],"output":{"name":"customaccess"}}],[11,"bitor","","",13,{"inputs":[{"name":"filemappingaccess"},{"name":"t"}],"output":{"name":"customaccess"}}],[11,"create","","",16,{"inputs":[{"name":"u64"}],"output":{"name":"result"}}],[11,"create_named","","",16,{"inputs":[{"name":"n"},{"name":"u64"}],"output":{"name":"createnamedresult"}}],[11,"create_from_file","","",16,{"inputs":[{"name":"f"}],"output":{"name":"result"}}],[11,"full","","",16,{"inputs":[{"name":"filemapping"}],"output":{"name":"result"}}],[11,"range","","",16,{"inputs":[{"name":"filemapping"},{"name":"u64"},{"name":"usize"}],"output":{"name":"result"}}],[11,"view","","",16,{"inputs":[{"name":"filemapping"}],"output":{"name":"fileviewbuilder"}}],[11,"flush","","",16,{"inputs":[{"name":"filemapping"}],"output":{"name":"result"}}],[11,"open_function","","",16,{"inputs":[],"output":{"name":"namedopenfunction"}}],[11,"default_open_options","","",16,{"inputs":[],"output":{"name":"namedopenoptions"}}],[11,"eq","","",14,{"inputs":[{"name":"fileviewwritemode"},{"name":"fileviewwritemode"}],"output":{"name":"bool"}}],[11,"clone","","",14,{"inputs":[{"name":"fileviewwritemode"}],"output":{"name":"fileviewwritemode"}}],[11,"fmt","","",14,{"inputs":[{"name":"fileviewwritemode"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"write_mode","","",17,{"inputs":[{"name":"fileviewbuilder"},{"name":"fileviewwritemode"}],"output":{"name":"fileviewbuilder"}}],[11,"executable","","",17,{"inputs":[{"name":"fileviewbuilder"},{"name":"bool"}],"output":{"name":"fileviewbuilder"}}],[11,"offset","","",17,{"inputs":[{"name":"fileviewbuilder"},{"name":"u64"}],"output":{"name":"fileviewbuilder"}}],[11,"size","","",17,{"inputs":[{"name":"fileviewbuilder"},{"name":"usize"}],"output":{"name":"fileviewbuilder"}}],[11,"map","","",17,{"inputs":[{"name":"fileviewbuilder"}],"output":{"name":"result"}}],[11,"as_ptr","","",18,null],[11,"as_mut_ptr","","",18,null],[11,"as_slice","","",18,null],[11,"as_mut_slice","","",18,null],[11,"size","","",18,{"inputs":[{"name":"fileview"}],"output":{"name":"usize"}}],[11,"flush_range","","",18,{"inputs":[{"name":"fileview"},{"name":"option"},{"name":"option"}],"output":{"name":"result"}}],[11,"flush","","",18,{"inputs":[{"name":"fileview"}],"output":{"name":"result"}}],[11,"drop","","",18,{"inputs":[{"name":"fileview"}],"output":null}],[0,"object","minwin","",null,null],[8,"Object","minwin::object","",null,null],[11,"set_inheritable","","",19,{"inputs":[{"name":"object"},{"name":"bool"}],"output":{"name":"result"}}],[11,"is_inheritable","","",19,{"inputs":[{"name":"object"}],"output":{"name":"result"}}],[11,"set_protected","","",19,{"inputs":[{"name":"object"},{"name":"bool"}],"output":{"name":"result"}}],[11,"is_protected","","",19,{"inputs":[{"name":"object"}],"output":{"name":"result"}}],[8,"ObjectExt","","",null,null],[11,"try_clone","","",20,{"inputs":[{"name":"objectext"}],"output":{"name":"result"}}],[11,"from_handle","","",20,{"inputs":[{"name":"handle"}],"output":{"name":"self"}}],[11,"into_handle","","",20,{"inputs":[{"name":"objectext"}],"output":{"name":"handle"}}],[11,"close","","",20,{"inputs":[{"name":"objectext"}],"output":{"name":"result"}}],[8,"Readable","","",null,null],[11,"read","","",21,null],[11,"read_overlapped","","",21,null],[8,"Writable","","",null,null],[11,"write","","",22,null],[11,"write_overlapped","","",22,null],[11,"flush","","",22,{"inputs":[{"name":"writable"}],"output":{"name":"result"}}],[0,"overlapped","minwin","",null,null],[3,"Overlapped","minwin::overlapped","",null,null],[11,"fmt","","",23,{"inputs":[{"name":"overlapped"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","",23,{"inputs":[],"output":{"name":"overlapped"}}],[11,"get","","",23,{"inputs":[{"name":"overlapped"}],"output":{"name":"overlapped"}}],[11,"set_offset","","",23,{"inputs":[{"name":"overlapped"},{"name":"u64"}],"output":null}],[11,"offset","","",23,{"inputs":[{"name":"overlapped"}],"output":{"name":"u64"}}],[11,"set_event","","",23,{"inputs":[{"name":"overlapped"},{"name":"option"}],"output":null}],[11,"event","","",23,{"inputs":[{"name":"overlapped"}],"output":{"name":"option"}}],[11,"default","","",23,{"inputs":[],"output":{"name":"overlapped"}}],[0,"pipe","minwin","",null,null],[3,"PipeBuilder","minwin::pipe","",null,null],[3,"ReadPipe","","",null,null],[3,"WritePipe","","",null,null],[3,"PipeInfo","","",null,null],[3,"PipeState","","",null,null],[3,"PeekInfo","","",null,null],[4,"PipeType","","",null,null],[13,"Byte","","",24,null],[13,"Message","","",24,null],[4,"PipeEnd","","",null,null],[13,"Client","","",25,null],[13,"Server","","",25,null],[4,"ReadMode","","",null,null],[13,"Byte","","",26,null],[13,"Message","","",26,null],[5,"create_anonymous_pipe","","",null,{"inputs":[],"output":{"name":"result"}}],[11,"new","","",27,{"inputs":[],"output":{"name":"pipebuilder"}}],[11,"size","","",27,{"inputs":[{"name":"pipebuilder"},{"name":"u32"}],"output":{"name":"pipebuilder"}}],[11,"create","","",27,{"inputs":[{"name":"pipebuilder"}],"output":{"name":"result"}}],[11,"fmt","","",28,{"inputs":[{"name":"readpipe"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from_raw_handle","","",28,{"inputs":[{"name":"rawhandle"}],"output":{"name":"readpipe"}}],[11,"as_raw_handle","","",28,{"inputs":[{"name":"readpipe"}],"output":{"name":"rawhandle"}}],[11,"into_raw_handle","","",28,{"inputs":[{"name":"readpipe"}],"output":{"name":"rawhandle"}}],[11,"read","","",28,null],[11,"fmt","","",29,{"inputs":[{"name":"writepipe"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from_raw_handle","","",29,{"inputs":[{"name":"rawhandle"}],"output":{"name":"writepipe"}}],[11,"as_raw_handle","","",29,{"inputs":[{"name":"writepipe"}],"output":{"name":"rawhandle"}}],[11,"into_raw_handle","","",29,{"inputs":[{"name":"writepipe"}],"output":{"name":"rawhandle"}}],[11,"write","","",29,null],[11,"flush","","",29,{"inputs":[{"name":"writepipe"}],"output":{"name":"result"}}],[8,"Pipe","","",null,null],[11,"kind","","",30,{"inputs":[{"name":"pipe"}],"output":{"name":"result"}}],[11,"transact","","",30,null],[11,"transact_overlapped","","",30,null],[11,"peek","","",30,{"inputs":[{"name":"pipe"},{"name":"option"}],"output":{"name":"result"}}],[11,"client_computer_name","","",30,{"inputs":[{"name":"pipe"}],"output":{"name":"result"}}],[11,"client_process_id","","",30,{"inputs":[{"name":"pipe"}],"output":{"name":"result"}}],[11,"client_session_id","","",30,{"inputs":[{"name":"pipe"}],"output":{"name":"result"}}],[11,"server_process_id","","",30,{"inputs":[{"name":"pipe"}],"output":{"name":"result"}}],[11,"server_session_id","","",30,{"inputs":[{"name":"pipe"}],"output":{"name":"result"}}],[11,"info","","",30,{"inputs":[{"name":"pipe"}],"output":{"name":"result"}}],[11,"state","","",30,{"inputs":[{"name":"pipe"}],"output":{"name":"result"}}],[11,"set_state","","",30,{"inputs":[{"name":"pipe"},{"name":"option"},{"name":"option"},{"name":"option"},{"name":"option"}],"output":{"name":"result"}}],[11,"clone","","",31,{"inputs":[{"name":"pipeinfo"}],"output":{"name":"pipeinfo"}}],[11,"fmt","","",31,{"inputs":[{"name":"pipeinfo"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"kind","","",31,{"inputs":[{"name":"pipeinfo"}],"output":{"name":"pipetype"}}],[11,"end","","",31,{"inputs":[{"name":"pipeinfo"}],"output":{"name":"pipeend"}}],[11,"out_buffer_size","","",31,{"inputs":[{"name":"pipeinfo"}],"output":{"name":"u32"}}],[11,"in_buffer_size","","",31,{"inputs":[{"name":"pipeinfo"}],"output":{"name":"u32"}}],[11,"max_instances","","",31,{"inputs":[{"name":"pipeinfo"}],"output":{"name":"option"}}],[11,"eq","","",24,{"inputs":[{"name":"pipetype"},{"name":"pipetype"}],"output":{"name":"bool"}}],[11,"clone","","",24,{"inputs":[{"name":"pipetype"}],"output":{"name":"pipetype"}}],[11,"fmt","","",24,{"inputs":[{"name":"pipetype"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",25,{"inputs":[{"name":"pipeend"},{"name":"pipeend"}],"output":{"name":"bool"}}],[11,"clone","","",25,{"inputs":[{"name":"pipeend"}],"output":{"name":"pipeend"}}],[11,"fmt","","",25,{"inputs":[{"name":"pipeend"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",32,{"inputs":[{"name":"pipestate"}],"output":{"name":"pipestate"}}],[11,"fmt","","",32,{"inputs":[{"name":"pipestate"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"blocking","","",32,{"inputs":[{"name":"pipestate"}],"output":{"name":"bool"}}],[11,"read_mode","","",32,{"inputs":[{"name":"pipestate"}],"output":{"name":"readmode"}}],[11,"current_instances","","",32,{"inputs":[{"name":"pipestate"}],"output":{"name":"u32"}}],[11,"max_collection_count","","",32,{"inputs":[{"name":"pipestate"}],"output":{"name":"u32"}}],[11,"collect_data_timeout","","",32,{"inputs":[{"name":"pipestate"}],"output":{"name":"u32"}}],[11,"user_name","","",32,{"inputs":[{"name":"pipestate"}],"output":{"name":"osstr"}}],[11,"eq","","",26,{"inputs":[{"name":"readmode"},{"name":"readmode"}],"output":{"name":"bool"}}],[11,"clone","","",26,{"inputs":[{"name":"readmode"}],"output":{"name":"readmode"}}],[11,"fmt","","",26,{"inputs":[{"name":"readmode"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",33,{"inputs":[{"name":"peekinfo"}],"output":{"name":"peekinfo"}}],[11,"fmt","","",33,{"inputs":[{"name":"peekinfo"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"bytes_read","","",33,{"inputs":[{"name":"peekinfo"}],"output":{"name":"u32"}}],[11,"bytes_left","","",33,{"inputs":[{"name":"peekinfo"}],"output":{"name":"u32"}}],[11,"bytes_message_left","","",33,{"inputs":[{"name":"peekinfo"}],"output":{"name":"u32"}}],[0,"process","minwin","",null,null],[3,"Process","minwin::process","",null,null],[11,"fmt","","",34,{"inputs":[{"name":"process"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from_raw_handle","","",34,{"inputs":[{"name":"rawhandle"}],"output":{"name":"process"}}],[11,"as_raw_handle","","",34,{"inputs":[{"name":"process"}],"output":{"name":"rawhandle"}}],[11,"into_raw_handle","","",34,{"inputs":[{"name":"process"}],"output":{"name":"rawhandle"}}],[0,"string","minwin","",null,null],[3,"NulError","minwin::string","An error used to indicate that a nul character was found in the input string.",null,null],[5,"wide_to_ansi","","",null,{"inputs":[{"name":"s"}],"output":{"name":"result"}}],[5,"wide_to_ansi_null","","",null,{"inputs":[{"name":"s"}],"output":{"name":"result"}}],[5,"ansi_to_wide","","",null,{"inputs":[{"name":"s"}],"output":{"name":"result"}}],[5,"ansi_to_wide_null","","",null,{"inputs":[{"name":"s"}],"output":{"name":"result"}}],[6,"WideString","","A type representing an owned wide string.",null,null],[6,"WideStr","","Slice into an wide string.",null,null],[6,"AnsiString","","A type representing an owned ANSI string encoded using the system default Windows ANSI code\npage.",null,null],[6,"AnsiStr","","Slice into an ANSI string.",null,null],[8,"ToWideString","","Trait for converting a string into a `WideString`.",null,null],[10,"to_wide_string","","Converts a string into a `WideString` not including a final nul character.",35,{"inputs":[{"name":"towidestring"}],"output":{"name":"widestring"}}],[11,"to_wide_string_null","","Converts a string into a `WideString` including a final nul character.",35,{"inputs":[{"name":"towidestring"}],"output":{"name":"result"}}],[8,"FromWideString","","",null,null],[10,"from_wide_string","","",36,{"inputs":[{"name":"w"}],"output":{"name":"self"}}],[11,"from_wide_string_null","","",36,{"inputs":[{"name":"w"}],"output":{"name":"self"}}],[8,"ToAnsiString","","",null,null],[10,"to_ansi_string","","",37,{"inputs":[{"name":"toansistring"}],"output":{"name":"result"}}],[10,"to_ansi_string_null","","",37,{"inputs":[{"name":"toansistring"}],"output":{"name":"result"}}],[11,"eq","","",38,{"inputs":[{"name":"nulerror"},{"name":"nulerror"}],"output":{"name":"bool"}}],[11,"ne","","",38,{"inputs":[{"name":"nulerror"},{"name":"nulerror"}],"output":{"name":"bool"}}],[11,"clone","","",38,{"inputs":[{"name":"nulerror"}],"output":{"name":"nulerror"}}],[11,"fmt","","",38,{"inputs":[{"name":"nulerror"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"nul_position","","",38,{"inputs":[{"name":"nulerror"}],"output":{"name":"usize"}}],[11,"into_vec","","",38,{"inputs":[{"name":"nulerror"}],"output":{"name":"vec"}}],[11,"description","","",38,{"inputs":[{"name":"nulerror"}],"output":{"name":"str"}}],[11,"fmt","","",38,{"inputs":[{"name":"nulerror"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from","std::io::error","",39,{"inputs":[{"name":"nulerror"}],"output":{"name":"error"}}],[11,"from_wide_string","std::ffi::os_str","",40,{"inputs":[{"name":"w"}],"output":{"name":"osstring"}}],[0,"sync","minwin","",null,null],[3,"EventBuilder","minwin::sync","A builder for creating a new `Event`.",null,null],[3,"Event","","A synchronization object whose state can be explicitly set to signaled.",null,null],[3,"MutexBuilder","","",null,null],[3,"Mutex","","",null,null],[3,"MutexGuard","","",null,null],[3,"SemaphoreBuilder","","",null,null],[3,"Semaphore","","",null,null],[3,"SemaphoreGuard","","",null,null],[3,"WaitableTimerBuilder","","",null,null],[3,"WaitableTimer","","",null,null],[4,"EventAccess","","",null,null],[13,"ModifyState","","",41,null],[4,"MutexAccess","","",null,null],[13,"ModifyState","","",42,null],[4,"LockError","","",null,null],[13,"Abandoned","","",43,null],[13,"Io","","",43,null],[4,"TryLockError","","",null,null],[13,"Abandoned","","",44,null],[13,"WouldBlock","","",44,null],[13,"Io","","",44,null],[4,"SemaphoreAccess","","",null,null],[13,"ModifyState","","",45,null],[4,"TryAcquireError","","",null,null],[13,"WouldBlock","","",46,null],[13,"Io","","",46,null],[4,"DueTime","","",null,null],[13,"Relative","","",47,null],[13,"Absolute","","",47,null],[4,"WaitableTimerAccess","","",null,null],[13,"ModifyState","","",48,null],[13,"QueryState","","",48,null],[11,"new","","Creates a new event builder with the default settings.",49,{"inputs":[],"output":{"name":"eventbuilder"}}],[11,"manual_reset","","The reset mode of the event.",49,{"inputs":[{"name":"eventbuilder"},{"name":"bool"}],"output":{"name":"eventbuilder"}}],[11,"initial_state","","The initial state of the event.",49,{"inputs":[{"name":"eventbuilder"},{"name":"bool"}],"output":{"name":"eventbuilder"}}],[11,"desired_access","","The desired access for the event object.",49,{"inputs":[{"name":"eventbuilder"},{"name":"a"}],"output":{"name":"eventbuilder"}}],[11,"fmt","","",50,{"inputs":[{"name":"event"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from_raw_handle","","",50,{"inputs":[{"name":"rawhandle"}],"output":{"name":"event"}}],[11,"as_raw_handle","","",50,{"inputs":[{"name":"event"}],"output":{"name":"rawhandle"}}],[11,"into_raw_handle","","",50,{"inputs":[{"name":"event"}],"output":{"name":"rawhandle"}}],[11,"eq","","",41,{"inputs":[{"name":"eventaccess"},{"name":"eventaccess"}],"output":{"name":"bool"}}],[11,"clone","","",41,{"inputs":[{"name":"eventaccess"}],"output":{"name":"eventaccess"}}],[11,"fmt","","",41,{"inputs":[{"name":"eventaccess"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"mask","","",41,{"inputs":[{"name":"eventaccess"}],"output":{"name":"u32"}}],[11,"all","","",41,{"inputs":[],"output":{"name":"customaccess"}}],[11,"bitor","","",41,{"inputs":[{"name":"eventaccess"},{"name":"t"}],"output":{"name":"customaccess"}}],[11,"create","","Creates a new anonymous event with default settings.",50,{"inputs":[],"output":{"name":"result"}}],[11,"create_named","","Creates a new named event with default settings.",50,{"inputs":[{"name":"n"}],"output":{"name":"createnamedresult"}}],[11,"set","","Sets the event object to the signaled state.",50,{"inputs":[{"name":"event"}],"output":{"name":"result"}}],[11,"reset","","Sets the event object to the nonsignaled state.",50,{"inputs":[{"name":"event"}],"output":{"name":"result"}}],[11,"open_function","","",50,{"inputs":[],"output":{"name":"namedopenfunction"}}],[11,"default_open_options","","",50,{"inputs":[],"output":{"name":"namedopenoptions"}}],[11,"new","","",51,{"inputs":[],"output":{"name":"mutexbuilder"}}],[11,"initial_owner","","",51,{"inputs":[{"name":"mutexbuilder"},{"name":"bool"}],"output":{"name":"mutexbuilder"}}],[11,"desired_access","","",51,{"inputs":[{"name":"mutexbuilder"},{"name":"a"}],"output":{"name":"mutexbuilder"}}],[11,"fmt","","",52,{"inputs":[{"name":"mutex"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from_raw_handle","","",52,{"inputs":[{"name":"rawhandle"}],"output":{"name":"mutex"}}],[11,"as_raw_handle","","",52,{"inputs":[{"name":"mutex"}],"output":{"name":"rawhandle"}}],[11,"into_raw_handle","","",52,{"inputs":[{"name":"mutex"}],"output":{"name":"rawhandle"}}],[11,"eq","","",42,{"inputs":[{"name":"mutexaccess"},{"name":"mutexaccess"}],"output":{"name":"bool"}}],[11,"clone","","",42,{"inputs":[{"name":"mutexaccess"}],"output":{"name":"mutexaccess"}}],[11,"fmt","","",42,{"inputs":[{"name":"mutexaccess"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"mask","","",42,{"inputs":[{"name":"mutexaccess"}],"output":{"name":"u32"}}],[11,"all","","",42,{"inputs":[],"output":{"name":"customaccess"}}],[11,"bitor","","",42,{"inputs":[{"name":"mutexaccess"},{"name":"t"}],"output":{"name":"customaccess"}}],[11,"create","","",52,{"inputs":[],"output":{"name":"result"}}],[11,"create_named","","",52,{"inputs":[{"name":"n"}],"output":{"name":"createnamedresult"}}],[11,"lock","","",52,{"inputs":[{"name":"mutex"}],"output":{"name":"result"}}],[11,"try_lock","","",52,{"inputs":[{"name":"mutex"}],"output":{"name":"result"}}],[11,"release","","",52,{"inputs":[{"name":"mutex"}],"output":{"name":"result"}}],[11,"guard","","",52,{"inputs":[{"name":"mutex"}],"output":{"name":"mutexguard"}}],[11,"open_function","","",52,{"inputs":[],"output":{"name":"namedopenfunction"}}],[11,"default_open_options","","",52,{"inputs":[],"output":{"name":"namedopenoptions"}}],[11,"fmt","","",53,{"inputs":[{"name":"mutexguard"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"drop","","",53,{"inputs":[{"name":"mutexguard"}],"output":null}],[11,"fmt","","",43,{"inputs":[{"name":"lockerror"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",43,{"inputs":[{"name":"lockerror"}],"output":{"name":"str"}}],[11,"cause","","",43,{"inputs":[{"name":"lockerror"}],"output":{"name":"option"}}],[11,"fmt","","",43,{"inputs":[{"name":"lockerror"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",44,{"inputs":[{"name":"trylockerror"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",44,{"inputs":[{"name":"trylockerror"}],"output":{"name":"str"}}],[11,"cause","","",44,{"inputs":[{"name":"trylockerror"}],"output":{"name":"option"}}],[11,"fmt","","",44,{"inputs":[{"name":"trylockerror"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","",54,{"inputs":[{"name":"i32"}],"output":{"name":"semaphorebuilder"}}],[11,"initial_count","","",54,{"inputs":[{"name":"semaphorebuilder"},{"name":"i32"}],"output":{"name":"semaphorebuilder"}}],[11,"desired_access","","",54,{"inputs":[{"name":"semaphorebuilder"},{"name":"a"}],"output":{"name":"semaphorebuilder"}}],[11,"fmt","","",55,{"inputs":[{"name":"semaphore"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from_raw_handle","","",55,{"inputs":[{"name":"rawhandle"}],"output":{"name":"semaphore"}}],[11,"as_raw_handle","","",55,{"inputs":[{"name":"semaphore"}],"output":{"name":"rawhandle"}}],[11,"into_raw_handle","","",55,{"inputs":[{"name":"semaphore"}],"output":{"name":"rawhandle"}}],[11,"eq","","",45,{"inputs":[{"name":"semaphoreaccess"},{"name":"semaphoreaccess"}],"output":{"name":"bool"}}],[11,"clone","","",45,{"inputs":[{"name":"semaphoreaccess"}],"output":{"name":"semaphoreaccess"}}],[11,"fmt","","",45,{"inputs":[{"name":"semaphoreaccess"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"mask","","",45,{"inputs":[{"name":"semaphoreaccess"}],"output":{"name":"u32"}}],[11,"all","","",45,{"inputs":[],"output":{"name":"customaccess"}}],[11,"bitor","","",45,{"inputs":[{"name":"semaphoreaccess"},{"name":"t"}],"output":{"name":"customaccess"}}],[11,"create","","",55,{"inputs":[{"name":"i32"}],"output":{"name":"result"}}],[11,"create_named","","",55,{"inputs":[{"name":"n"},{"name":"i32"}],"output":{"name":"createnamedresult"}}],[11,"acquire","","",55,{"inputs":[{"name":"semaphore"}],"output":{"name":"result"}}],[11,"try_acquire","","",55,{"inputs":[{"name":"semaphore"}],"output":{"name":"result"}}],[11,"release","","",55,{"inputs":[{"name":"semaphore"},{"name":"i32"}],"output":{"name":"result"}}],[11,"guard","","",55,{"inputs":[{"name":"semaphore"},{"name":"i32"}],"output":{"name":"semaphoreguard"}}],[11,"open_function","","",55,{"inputs":[],"output":{"name":"namedopenfunction"}}],[11,"default_open_options","","",55,{"inputs":[],"output":{"name":"namedopenoptions"}}],[11,"fmt","","",56,{"inputs":[{"name":"semaphoreguard"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"drop","","",56,{"inputs":[{"name":"semaphoreguard"}],"output":null}],[11,"fmt","","",46,{"inputs":[{"name":"tryacquireerror"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",46,{"inputs":[{"name":"tryacquireerror"}],"output":{"name":"str"}}],[11,"cause","","",46,{"inputs":[{"name":"tryacquireerror"}],"output":{"name":"option"}}],[11,"fmt","","",46,{"inputs":[{"name":"tryacquireerror"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","",57,{"inputs":[],"output":{"name":"waitabletimerbuilder"}}],[11,"manual_reset","","",57,{"inputs":[{"name":"waitabletimerbuilder"},{"name":"bool"}],"output":{"name":"waitabletimerbuilder"}}],[11,"desired_access","","",57,{"inputs":[{"name":"waitabletimerbuilder"},{"name":"a"}],"output":{"name":"waitabletimerbuilder"}}],[11,"as_filetime","","",47,{"inputs":[{"name":"duetime"}],"output":{"name":"i64"}}],[11,"fmt","","",58,{"inputs":[{"name":"waitabletimer"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from_raw_handle","","",58,{"inputs":[{"name":"rawhandle"}],"output":{"name":"waitabletimer"}}],[11,"as_raw_handle","","",58,{"inputs":[{"name":"waitabletimer"}],"output":{"name":"rawhandle"}}],[11,"into_raw_handle","","",58,{"inputs":[{"name":"waitabletimer"}],"output":{"name":"rawhandle"}}],[11,"eq","","",48,{"inputs":[{"name":"waitabletimeraccess"},{"name":"waitabletimeraccess"}],"output":{"name":"bool"}}],[11,"clone","","",48,{"inputs":[{"name":"waitabletimeraccess"}],"output":{"name":"waitabletimeraccess"}}],[11,"fmt","","",48,{"inputs":[{"name":"waitabletimeraccess"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"mask","","",48,{"inputs":[{"name":"waitabletimeraccess"}],"output":{"name":"u32"}}],[11,"all","","",48,{"inputs":[],"output":{"name":"customaccess"}}],[11,"bitor","","",48,{"inputs":[{"name":"waitabletimeraccess"},{"name":"t"}],"output":{"name":"customaccess"}}],[11,"create","","",58,{"inputs":[],"output":{"name":"result"}}],[11,"create_named","","",58,{"inputs":[{"name":"n"}],"output":{"name":"createnamedresult"}}],[11,"set","","",58,{"inputs":[{"name":"waitabletimer"},{"name":"duetime"},{"name":"option"}],"output":{"name":"result"}}],[11,"set_with_completion_routine","","",58,{"inputs":[{"name":"waitabletimer"},{"name":"duetime"},{"name":"option"},{"name":"f"}],"output":{"name":"result"}}],[11,"cancel","","",58,{"inputs":[{"name":"waitabletimer"}],"output":{"name":"result"}}],[11,"open_function","","",58,{"inputs":[],"output":{"name":"namedopenfunction"}}],[11,"default_open_options","","",58,{"inputs":[],"output":{"name":"namedopenoptions"}}],[0,"waitable","minwin","",null,null],[4,"WaitError","minwin::waitable","",null,null],[13,"Abandoned","","",59,null],[13,"Timeout","","",59,null],[13,"Io","","",59,null],[5,"wait_for_all","","",null,null],[5,"wait_for_any","","",null,null],[6,"WaitResult","","",null,null],[8,"Waitable","","",null,null],[11,"wait","","",60,{"inputs":[{"name":"waitable"}],"output":{"name":"waitresult"}}],[11,"wait_timeout","","",60,{"inputs":[{"name":"waitable"},{"name":"duration"}],"output":{"name":"waitresult"}}],[11,"fmt","","",59,{"inputs":[{"name":"waiterror"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",59,{"inputs":[{"name":"waiterror"}],"output":{"name":"str"}}],[11,"cause","","",59,{"inputs":[{"name":"waiterror"}],"output":{"name":"option"}}],[11,"fmt","","",59,{"inputs":[{"name":"waiterror"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from","","",59,{"inputs":[{"name":"error"}],"output":{"name":"waiterror"}}],[0,"prelude","minwin","",null,null]],"paths":[[3,"CustomAccess"],[4,"GenericAccess"],[4,"StandardAccess"],[8,"Access"],[8,"CombinableAccess"],[3,"MaximumAccess"],[3,"SystemSecurityAccess"],[3,"Handle"],[4,"CreateNamedError"],[8,"NamedBuilder"],[8,"NamedObject"],[3,"NamedOpenOptions"],[4,"FileMappingAttribute"],[4,"FileMappingAccess"],[4,"FileViewWriteMode"],[3,"FileMappingBuilder"],[3,"FileMapping"],[3,"FileViewBuilder"],[3,"FileView"],[8,"Object"],[8,"ObjectExt"],[8,"Readable"],[8,"Writable"],[3,"Overlapped"],[4,"PipeType"],[4,"PipeEnd"],[4,"ReadMode"],[3,"PipeBuilder"],[3,"ReadPipe"],[3,"WritePipe"],[8,"Pipe"],[3,"PipeInfo"],[3,"PipeState"],[3,"PeekInfo"],[3,"Process"],[8,"ToWideString"],[8,"FromWideString"],[8,"ToAnsiString"],[3,"NulError"],[3,"Error"],[3,"OsString"],[4,"EventAccess"],[4,"MutexAccess"],[4,"LockError"],[4,"TryLockError"],[4,"SemaphoreAccess"],[4,"TryAcquireError"],[4,"DueTime"],[4,"WaitableTimerAccess"],[3,"EventBuilder"],[3,"Event"],[3,"MutexBuilder"],[3,"Mutex"],[3,"MutexGuard"],[3,"SemaphoreBuilder"],[3,"Semaphore"],[3,"SemaphoreGuard"],[3,"WaitableTimerBuilder"],[3,"WaitableTimer"],[4,"WaitError"],[8,"Waitable"]]};
initSearch(searchIndex);
