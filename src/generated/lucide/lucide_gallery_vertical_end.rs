use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M7 2h10" ></ path > < path d = "M5 6h14" ></ path > < rect height = "12" x = "3" width = "18" y = "10" rx = "2" ></ rect > < / > } } pub const LUCIDE_GALLERY_VERTICAL_END : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;