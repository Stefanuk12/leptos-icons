use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "6" x2 = "10" y1 = "12" y2 = "12" ></ line > < line x2 = "8" y2 = "14" y1 = "10" x1 = "8" ></ line > < line x1 = "15" y2 = "13" y1 = "13" x2 = "15.01" ></ line > < line x1 = "18" x2 = "18.01" y1 = "11" y2 = "11" ></ line > < rect x = "2" width = "20" height = "12" y = "6" rx = "2" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;