use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v1" ></ path > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < rect y = "13" height = "5" x = "2" width = "8" rx = "1" ></ rect > < path d = "M8 13v-2a2 2 0 1 0-4 0v2" ></ path > < / > } } pub const LUCIDE_FILE_LOCK_2 : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;