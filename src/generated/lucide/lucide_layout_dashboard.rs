use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" width = "7" height = "9" y = "3" rx = "1" ></ rect > < rect x = "14" width = "7" height = "5" y = "3" rx = "1" ></ rect > < rect x = "14" rx = "1" width = "7" height = "9" y = "12" ></ rect > < rect x = "3" y = "16" rx = "1" width = "7" height = "5" ></ rect > < / > } } pub const LUCIDE_LAYOUT_DASHBOARD : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;