use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 7v10" ></ path > < path d = "M6 5v14" ></ path > < rect y = "3" height = "18" width = "12" x = "10" rx = "2" ></ rect > < / > } } pub const LUCIDE_GALLERY_HORIZONTAL_END : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor")] } ;