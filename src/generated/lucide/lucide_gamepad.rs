use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "6" y2 = "12" x2 = "10" y1 = "12" ></ line > < line x1 = "8" x2 = "8" y1 = "10" y2 = "14" ></ line > < line x2 = "15.01" x1 = "15" y2 = "13" y1 = "13" ></ line > < line x1 = "18" y2 = "11" x2 = "18.01" y1 = "11" ></ line > < rect x = "2" width = "20" height = "12" rx = "2" y = "6" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24")] } ;