use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v1" ></ path > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < rect width = "8" height = "5" y = "13" rx = "1" x = "2" ></ rect > < path d = "M8 13v-2a2 2 0 1 0-4 0v2" ></ path > < / > } } pub const LUCIDE_FILE_LOCK_2 : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2")] } ;