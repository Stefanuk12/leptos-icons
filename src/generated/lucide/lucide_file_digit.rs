use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v4" ></ path > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < rect height = "6" x = "2" y = "12" rx = "2" width = "4" ></ rect > < path d = "M10 12h2v6" ></ path > < path d = "M10 18h4" ></ path > < / > } } pub const LUCIDE_FILE_DIGIT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("fill" , "none") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;