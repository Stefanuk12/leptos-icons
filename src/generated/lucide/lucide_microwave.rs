use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" x = "2" height = "15" y = "4" width = "20" ></ rect > < rect width = "8" height = "7" y = "8" x = "6" rx = "1" ></ rect > < path d = "M18 8v7" ></ path > < path d = "M6 19v2" ></ path > < path d = "M18 19v2" ></ path > < / > } } pub const LUCIDE_MICROWAVE : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24")] } ;