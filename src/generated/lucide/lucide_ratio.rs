use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "6" height = "20" y = "2" width = "12" rx = "2" ></ rect > < rect width = "20" y = "6" x = "2" rx = "2" height = "12" ></ rect > < / > } } pub const LUCIDE_RATIO : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;