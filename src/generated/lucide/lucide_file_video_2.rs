use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v4" ></ path > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < rect width = "8" y = "12" height = "6" rx = "1" x = "2" ></ rect > < path d = "m10 15.5 4 2.5v-6l-4 2.5" ></ path > < / > } } pub const LUCIDE_FILE_VIDEO_2 : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;