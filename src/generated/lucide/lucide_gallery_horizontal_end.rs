use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 7v10" ></ path > < path d = "M6 5v14" ></ path > < rect width = "12" y = "3" height = "18" x = "10" rx = "2" ></ rect > < / > } } pub const LUCIDE_GALLERY_HORIZONTAL_END : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24")] } ;