use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "6" width = "12" height = "20" y = "2" rx = "2" ></ rect > < rect rx = "2" width = "20" y = "6" x = "2" height = "12" ></ rect > < / > } } pub const LUCIDE_RATIO : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24")] } ;