use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M7 2h10" ></ path > < path d = "M5 6h14" ></ path > < rect y = "10" width = "18" height = "12" rx = "2" x = "3" ></ rect > < / > } } pub const LUCIDE_GALLERY_VERTICAL_END : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;