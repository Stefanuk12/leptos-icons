use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 9v6" ></ path > < path d = "M16 15v6" ></ path > < path d = "M16 3v6" ></ path > < path d = "M3 15h18" ></ path > < path d = "M3 9h18" ></ path > < path d = "M8 15v6" ></ path > < path d = "M8 3v6" ></ path > < / > } } pub const LucideBrickWall : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;