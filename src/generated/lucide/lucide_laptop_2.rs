use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" width = "18" height = "12" x = "3" y = "4" rx = "2" ></ rect > < line x2 = "22" y1 = "20" y2 = "20" x1 = "2" ></ line > < / > } } pub const LucideLaptop2 : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24")] } ;