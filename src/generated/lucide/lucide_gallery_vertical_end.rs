use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M7 2h10" ></ path > < path d = "M5 6h14" ></ path > < rect width = "18" y = "10" rx = "2" x = "3" height = "12" ></ rect > < / > } } pub const LUCIDE_GALLERY_VERTICAL_END : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;