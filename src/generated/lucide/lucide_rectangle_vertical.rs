use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "12" y = "2" rx = "2" x = "6" height = "20" ></ rect > < / > } } pub const LUCIDE_RECTANGLE_VERTICAL : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;