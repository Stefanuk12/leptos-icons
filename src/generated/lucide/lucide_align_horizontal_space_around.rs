use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "7" x = "9" rx = "2" height = "10" width = "6" ></ rect > < path d = "M4 22V2" ></ path > < path d = "M20 22V2" ></ path > < / > } } pub const LUCIDE_ALIGN_HORIZONTAL_SPACE_AROUND : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;