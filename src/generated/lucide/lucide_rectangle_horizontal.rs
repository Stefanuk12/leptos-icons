use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "12" x = "2" rx = "2" y = "6" width = "20" ></ rect > < / > } } pub const LUCIDE_RECTANGLE_HORIZONTAL : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;