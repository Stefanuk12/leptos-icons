use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4.499 11.998h15m-7.5-6.75h.008v.008h-.008v-.008Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0ZM12 18.751h.007v.007H12v-.007Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Z" stroke - linejoin = "round" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_DIVIDE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "1.5") , ("aria-hidden" , "true") , ("fill" , "none") , ("data-slot" , "icon")] } ;