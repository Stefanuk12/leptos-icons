use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "14" x = "14" width = "8" height = "8" rx = "2" ></ rect > < rect rx = "2" width = "8" x = "2" y = "2" height = "8" ></ rect > < path d = "M7 14v1a2 2 0 0 0 2 2h1" ></ path > < path d = "M14 7h1a2 2 0 0 1 2 2v1" ></ path > < / > } } pub const LUCIDE_SEND_TO_BACK : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;