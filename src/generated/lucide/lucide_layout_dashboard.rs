use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "9" y = "3" x = "3" width = "7" rx = "1" ></ rect > < rect y = "3" x = "14" rx = "1" height = "5" width = "7" ></ rect > < rect rx = "1" x = "14" width = "7" height = "9" y = "12" ></ rect > < rect x = "3" rx = "1" height = "5" width = "7" y = "16" ></ rect > < / > } } pub const LUCIDE_LAYOUT_DASHBOARD : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;