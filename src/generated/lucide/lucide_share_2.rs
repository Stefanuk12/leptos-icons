use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "5" r = "3" cx = "18" ></ circle > < circle cx = "6" cy = "12" r = "3" ></ circle > < circle r = "3" cx = "18" cy = "19" ></ circle > < line x2 = "15.42" y2 = "17.49" x1 = "8.59" y1 = "13.51" ></ line > < line y2 = "10.49" x1 = "15.41" x2 = "8.59" y1 = "6.51" ></ line > < / > } } pub const LucideShare2 : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;