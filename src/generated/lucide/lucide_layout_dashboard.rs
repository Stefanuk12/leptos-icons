use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "9" width = "7" rx = "1" x = "3" y = "3" ></ rect > < rect rx = "1" width = "7" x = "14" height = "5" y = "3" ></ rect > < rect y = "12" width = "7" rx = "1" height = "9" x = "14" ></ rect > < rect x = "3" y = "16" width = "7" height = "5" rx = "1" ></ rect > < / > } } pub const LUCIDE_LAYOUT_DASHBOARD : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;