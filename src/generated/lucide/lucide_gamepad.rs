use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "6" y1 = "12" x2 = "10" y2 = "12" ></ line > < line x1 = "8" x2 = "8" y1 = "10" y2 = "14" ></ line > < line x1 = "15" x2 = "15.01" y1 = "13" y2 = "13" ></ line > < line x2 = "18.01" y2 = "11" x1 = "18" y1 = "11" ></ line > < rect y = "6" height = "12" width = "20" rx = "2" x = "2" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;