use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "20" x = "6" y = "2" width = "12" ></ rect > < / > } } pub const LUCIDE_RECTANGLE_VERTICAL : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;