use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" width = "7" height = "9" y = "3" x = "3" ></ rect > < rect height = "5" x = "14" y = "3" width = "7" rx = "1" ></ rect > < rect width = "7" height = "9" x = "14" y = "12" rx = "1" ></ rect > < rect width = "7" x = "3" height = "5" y = "16" rx = "1" ></ rect > < / > } } pub const LUCIDE_LAYOUT_DASHBOARD : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round")] } ;