use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "9" height = "10" width = "6" y = "7" rx = "2" ></ rect > < path d = "M4 22V2" ></ path > < path d = "M20 22V2" ></ path > < / > } } pub const LUCIDE_ALIGN_HORIZONTAL_SPACE_AROUND : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor")] } ;