use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "7" height = "9" y = "3" rx = "1" x = "3" ></ rect > < rect width = "7" y = "3" x = "14" rx = "1" height = "5" ></ rect > < rect width = "7" y = "12" rx = "1" height = "9" x = "14" ></ rect > < rect rx = "1" width = "7" y = "16" height = "5" x = "3" ></ rect > < / > } } pub const LUCIDE_LAYOUT_DASHBOARD : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;