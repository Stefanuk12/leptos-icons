use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M7 2h10" ></ path > < path d = "M5 6h14" ></ path > < rect height = "12" rx = "2" width = "18" x = "3" y = "10" ></ rect > < / > } } pub const LUCIDE_GALLERY_VERTICAL_END : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;