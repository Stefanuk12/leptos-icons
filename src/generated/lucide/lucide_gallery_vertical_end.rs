use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M7 2h10" ></ path > < path d = "M5 6h14" ></ path > < rect y = "10" height = "12" rx = "2" width = "18" x = "3" ></ rect > < / > } } pub const LUCIDE_GALLERY_VERTICAL_END : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;