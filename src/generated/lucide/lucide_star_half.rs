use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 17.8 5.8 21 7 14.1 2 9.3l7-1L12 2" ></ path > < / > } } pub const LUCIDE_STAR_HALF : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;