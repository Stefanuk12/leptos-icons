use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" x = "2" height = "12" rx = "2" y = "6" ></ rect > < / > } } pub const LUCIDE_RECTANGLE_HORIZONTAL : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;