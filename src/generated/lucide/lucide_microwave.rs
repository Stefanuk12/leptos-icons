use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" x = "2" y = "4" height = "15" rx = "2" ></ rect > < rect rx = "1" y = "8" width = "8" height = "7" x = "6" ></ rect > < path d = "M18 8v7" ></ path > < path d = "M6 19v2" ></ path > < path d = "M18 19v2" ></ path > < / > } } pub const LUCIDE_MICROWAVE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;