use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "12" x = "6" height = "20" rx = "2" y = "2" ></ rect > < rect width = "20" height = "12" rx = "2" y = "6" x = "2" ></ rect > < / > } } pub const LUCIDE_RATIO : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-width" , "2")] } ;