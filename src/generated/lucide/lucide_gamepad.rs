use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "6" y1 = "12" x2 = "10" y2 = "12" ></ line > < line x2 = "8" y1 = "10" x1 = "8" y2 = "14" ></ line > < line y2 = "13" x2 = "15.01" y1 = "13" x1 = "15" ></ line > < line y1 = "11" x1 = "18" x2 = "18.01" y2 = "11" ></ line > < rect width = "20" height = "12" x = "2" y = "6" rx = "2" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;