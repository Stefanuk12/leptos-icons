use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "14" width = "8" x = "14" rx = "2" height = "8" ></ rect > < rect y = "2" height = "8" width = "8" x = "2" rx = "2" ></ rect > < path d = "M7 14v1a2 2 0 0 0 2 2h1" ></ path > < path d = "M14 7h1a2 2 0 0 1 2 2v1" ></ path > < / > } } pub const LUCIDE_SEND_TO_BACK : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;