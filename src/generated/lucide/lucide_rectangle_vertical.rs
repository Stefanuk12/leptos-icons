use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "6" y = "2" height = "20" rx = "2" width = "12" ></ rect > < / > } } pub const LUCIDE_RECTANGLE_VERTICAL : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2")] } ;