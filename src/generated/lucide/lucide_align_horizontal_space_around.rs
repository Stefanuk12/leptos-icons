use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "7" height = "10" width = "6" x = "9" rx = "2" ></ rect > < path d = "M4 22V2" ></ path > < path d = "M20 22V2" ></ path > < / > } } pub const LUCIDE_ALIGN_HORIZONTAL_SPACE_AROUND : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;