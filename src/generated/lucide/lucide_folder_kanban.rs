use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z" ></ path > < path d = "M8 10v4" ></ path > < path d = "M12 10v2" ></ path > < path d = "M16 10v6" ></ path > < / > } } pub const LUCIDE_FOLDER_KANBAN : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24")] } ;