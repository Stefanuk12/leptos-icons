use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "14" y = "14" height = "8" width = "8" rx = "2" ></ rect > < rect rx = "2" y = "2" x = "2" width = "8" height = "8" ></ rect > < path d = "M7 14v1a2 2 0 0 0 2 2h1" ></ path > < path d = "M14 7h1a2 2 0 0 1 2 2v1" ></ path > < / > } } pub const LUCIDE_SEND_TO_BACK : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;