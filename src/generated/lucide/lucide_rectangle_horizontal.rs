use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" rx = "2" width = "20" height = "12" y = "6" ></ rect > < / > } } pub const LUCIDE_RECTANGLE_HORIZONTAL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;