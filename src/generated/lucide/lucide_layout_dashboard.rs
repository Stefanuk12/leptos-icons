use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" width = "7" height = "9" x = "3" rx = "1" ></ rect > < rect height = "5" y = "3" rx = "1" x = "14" width = "7" ></ rect > < rect rx = "1" width = "7" height = "9" x = "14" y = "12" ></ rect > < rect height = "5" width = "7" x = "3" rx = "1" y = "16" ></ rect > < / > } } pub const LUCIDE_LAYOUT_DASHBOARD : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;