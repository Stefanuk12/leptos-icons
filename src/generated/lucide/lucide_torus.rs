use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < ellipse ry = "2" cx = "12" rx = "3" cy = "11" ></ ellipse > < ellipse ry = "8.5" cx = "12" cy = "12.5" rx = "10" ></ ellipse > < / > } } pub const LUCIDE_TORUS : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24")] } ;